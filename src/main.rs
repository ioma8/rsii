mod config;

use clipboard::{ClipboardContext, ClipboardProvider};
use openai_api_rs::v1::api::OpenAIClient;
use openai_api_rs::v1::chat_completion;
use openai_api_rs::v1::chat_completion::ChatCompletionRequest;
use openai_api_rs::v1::chat_completion::Tool;
use openai_api_rs::v1::chat_completion::ToolCall;
use openai_api_rs::v1::chat_completion::ToolCallFunction;
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

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 && args[1] == "--version" {
        println!("Version: {}", VERSION);
        return;
    }

    let user_message = args.get(1..).map_or_else(|| "".to_string(), |msg| msg.join(" "));
    let config = match config::load_config() {
        Ok(cfg) => cfg,
        Err(e) => {
            eprintln!("Error loading config: {}", e);
            return;
        }
    };

    let client = OpenAIClient::builder()
        .with_api_key(&config.api_key)
        .build()
        .expect("Failed to create client");

    let user_arch_str = match get_user_architecture() {
        Ok(arch) => arch,
        Err(e) => {
            eprintln!("Error retrieving system info: {}", e);
            return;
        }
    };

    let total_prompt = format!(
        "{} Users system info: {} \n User query:\n{}",
        config.system_prompt, user_arch_str, user_message
    );

    let req = ChatCompletionRequest::new(
        config.model.clone(),
        vec![chat_completion::ChatCompletionMessage {
            role: chat_completion::MessageRole::user,
            content: chat_completion::Content::Text(total_prompt),
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
    if let Err(e) = handle_ai_response(client, req).await {
        eprintln!("AI response error: {}", e);
    }
}

fn get_user_architecture() -> Result<String, std::io::Error> {
    let output: Output = Command::new("uname").arg("-a").output()?;
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

fn create_tool_function() -> Function {
    let mut properties = HashMap::new();
    properties.insert(
        "command".to_string(),
        Box::new(JSONSchemaDefine {
            schema_type: Some(JSONSchemaType::String),
            description: Some("The command to be executed".to_string()),
            enum_values: None,
            properties: None,
            required: None,
            items: None,
        }),
    );
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
    client: OpenAIClient,
    req: ChatCompletionRequest,
) -> Result<(), Box<dyn std::error::Error>> {
    let result = client.chat_completion(req).await?;
    if let Some(tool_calls) = &result.choices[0].message.tool_calls {
        for tool_call in tool_calls {
            if let ToolCall {
                id: _,
                r#type: _,
                function:
                    ToolCallFunction {
                        name: Some(tool_name),
                        arguments: Some(tool_arguments),
                    },
            } = tool_call
            {
                if tool_name == "call_command" {
                    let command_value: Value = serde_json::from_str(tool_arguments)?;
                    if let Some(command_str) = command_value["command"].as_str() {
                        let mut ctx: ClipboardContext = ClipboardProvider::new()?;
                        ctx.set_contents(command_str.to_string())?;
                        println!("Command is ready");
                        paste_command();
                    }
                }
            }
        }
    }
    Ok(())
}

fn paste_command() {
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

    Command::new("sh")
        .arg("-c")
        .arg(script)
        .spawn()
        .expect("Failed to run paste script");
}
