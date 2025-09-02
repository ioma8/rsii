mod config;

use clipboard::{ClipboardContext, ClipboardProvider};
use clap::Parser;
use openai_api_rs::v1::api::OpenAIClient;
use openai_api_rs::v1::chat_completion;
use openai_api_rs::v1::chat_completion::ChatCompletionRequest;
use openai_api_rs::v1::chat_completion::Tool;
use openai_api_rs::v1::chat_completion::ToolCall;
use openai_api_rs::v1::chat_completion::ToolType;
use openai_api_rs::v1::types::Function;
use openai_api_rs::v1::types::FunctionParameters;
use openai_api_rs::v1::types::JSONSchemaDefine;
use openai_api_rs::v1::types::JSONSchemaType;
use serde_json::Value;
use std::collections::HashMap;
use std::env;
use std::process::{Command, Output};

include!(concat!(env!("OUT_DIR"), "/version.rs"));

#[derive(Parser, Debug)]
#[command(name = "rsii", version = VERSION, disable_help_subcommand = true)]
struct Cli {
    #[arg(long, short = 'v')]
    verbose: bool,

    #[arg(trailing_var_arg = true, help = "Your natural language query")]
    query: Vec<String>,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    let user_message = cli.query.join(" ");
    if user_message.trim().is_empty() {
        eprintln!("Usage: rsii \"your query here\"");
        return;
    }

    let config = match config::load_config() {
        Ok(cfg) => cfg,
        Err(e) => {
            eprintln!("Error loading config: {}", e);
            return;
        }
    };

    let mut client = OpenAIClient::builder()
        .with_api_key(&config.api_key)
        .build()
        .expect("Failed to create client");

    let user_arch_str = match system_info() {
        Ok(arch) => arch,
        Err(e) => {
            eprintln!("Error retrieving system info: {}", e);
            return;
        }
    };

    let prompt = build_prompt(&config.system_prompt, &user_arch_str, &user_message);
    if cli.verbose {
        println!("Prompt: {}", prompt);
    }

    let req = ChatCompletionRequest::new(
        config.model.clone(),
        vec![chat_completion::ChatCompletionMessage {
            role: chat_completion::MessageRole::user,
            content: chat_completion::Content::Text(prompt),
            name: None,
            tool_call_id: None,
            tool_calls: None,
        }],
    )
    .tool_choice(chat_completion::ToolChoiceType::Required)
    .tools(vec![Tool {
        r#type: ToolType::Function,
        function: create_tool_function(),
    }]);

    println!("Getting AI response...");
    if let Err(e) = handle_ai_response(&mut client, req).await {
        eprintln!("AI response error: {}", e);
    }
}

// clap handles argument parsing

fn build_prompt(system_prompt: &str, system_info: &str, user_message: &str) -> String {
    format!(
        "{} Users system info: {} \n User query:\n{}",
        system_prompt, system_info, user_message
    )
}

fn system_info() -> Result<String, std::io::Error> {
    let output: Output = Command::new("uname").arg("-a").output()?;
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

fn create_tool_function() -> Function {
    let properties: HashMap<String, Box<JSONSchemaDefine>> = HashMap::from([(
        "command".to_string(),
        Box::new(JSONSchemaDefine {
            schema_type: Some(JSONSchemaType::String),
            description: Some("The command to be executed".to_string()),
            enum_values: None,
            properties: None,
            required: None,
            items: None,
        }),
    )]);
    Function {
        name: "call_command".to_string(),
        description: Some("calls the given command for user".to_string()),
        parameters: FunctionParameters {
            schema_type: JSONSchemaType::Object,
            required: Some(vec!["command".to_string()]),
            properties: Some(properties),
        },
    }
}

async fn handle_ai_response(
    client: &mut OpenAIClient,
    req: ChatCompletionRequest,
) -> Result<(), Box<dyn std::error::Error>> {
    let result = client.chat_completion(req).await?;
    let Some(tool_calls) = &result.choices[0].message.tool_calls else { return Ok(()); };

    for tc in tool_calls {
        let Some(cmd) = extract_command_from_tool_call(tc)? else { continue; };
        copy_to_clipboard(&cmd)?;
        println!("Command copied to clipboard");
        if let Err(e) = paste_command() { eprintln!("Paste simulation failed: {}", e); }
    }
    Ok(())
}

fn extract_command_from_tool_call(tc: &ToolCall) -> Result<Option<String>, Box<dyn std::error::Error>> {
    let function = &tc.function;
    let is_call_command = function.name.as_deref() == Some("call_command");
    if !is_call_command { return Ok(None); }

    let Some(args) = &function.arguments else { return Ok(None) };
    let parsed: Value = serde_json::from_str(args)?;
    let Some(cmd) = parsed["command"].as_str() else { return Ok(None) };
    Ok(Some(cmd.to_string()))
}

fn paste_command() -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(target_os = "macos")]
    let script = r#"
    osascript -e 'delay 0' -e 'tell application "System Events" to keystroke "v" using command down'
    "#;
    #[cfg(target_os = "linux")]
    let script = r#"
    xdotool key --delay 1000 ctrl+v
    "#;
    #[cfg(target_os = "windows")]
    let script = r#"
    powershell -command "Start-Sleep -Seconds 1; Add-Type -AssemblyName System.Windows.Forms; [System.Windows.Forms.SendKeys]::SendWait('^v')"
    "#;

    Command::new("sh").arg("-c").arg(script).spawn()?;
    Ok(())
}

fn copy_to_clipboard(contents: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut ctx: ClipboardContext = ClipboardProvider::new()?;
    ctx.set_contents(contents.to_string())?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::{Parser, error::ErrorKind};
    use tokio;
    use std::boxed::Box;

    #[test]
    fn test_system_info() {
        let output = system_info().expect("Failed to get system info");
        assert!(!output.is_empty(), "The output should not be empty");
    }

    #[test]
    fn test_load_config() {
        let config = config::load_config().expect("Failed to load config");
        assert!(!config.model.is_empty(), "Model should not be empty");
        assert!(!config.api_key.is_empty(), "API Key should not be empty");
        assert!(!config.system_prompt.is_empty(), "System prompt should not be empty");
    }

    #[tokio::test]
    async fn test_handle_ai_response() -> Result<(), Box<dyn std::error::Error>> {
        let mut client = OpenAIClient::builder()
            .with_api_key("dummy_api_key")
            .build()
            .expect("Failed to create client");

        let req = ChatCompletionRequest::new(
            "dummy_model".to_string(),
            vec![chat_completion::ChatCompletionMessage {
                role: chat_completion::MessageRole::user,
                content: chat_completion::Content::Text("test query".to_string()),
                name: None,
                tool_call_id: None,
                tool_calls: None,
            }],
        );

        let result = handle_ai_response(&mut client, req).await;
        assert!(result.is_err(), "AI Response should fail with dummy API key");
        Ok(())
    }

    #[test]
    fn test_create_tool_function() {
        let function = create_tool_function();
        assert_eq!(function.name, "call_command");
        assert!(function.parameters.properties.is_some(), "Properties should be defined");
    }

    #[test]
    fn test_build_prompt() {
        let prompt = build_prompt("SYS", "Darwin 23.5.0", "list files");
        assert!(prompt.contains("SYS"));
        assert!(prompt.contains("Darwin"));
        assert!(prompt.contains("list files"));
    }

    #[test]
    fn test_clap_parse_basic_query() {
        let args = ["rsii", "echo", "hello"];
        let cli = Cli::parse_from(&args);
        assert_eq!(cli.query.join(" "), "echo hello");
        assert!(!cli.verbose);
    }

    #[test]
    fn test_clap_parse_version() {
        let args = ["rsii", "--version"]; 
        let res = Cli::try_parse_from(&args);
        assert!(res.is_err());
        let err = res.unwrap_err();
        assert_eq!(err.kind(), ErrorKind::DisplayVersion);
    }

    #[test]
    fn test_clap_parse_verbose_flag() {
        let args = ["rsii", "--verbose", "ls", "-la"];
        let cli = Cli::parse_from(&args);
        assert_eq!(cli.query.join(" "), "ls -la");
        assert!(cli.verbose);
    }
}
