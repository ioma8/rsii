use std::env;
use std::fs;
use std::process::Command;

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

include!(concat!(env!("OUT_DIR"), "/version.rs"));

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 && args[1] == "--version" {
        println!("Version: {}", VERSION);
        return;
    }

    let default_user_message = String::from("");
    let user_message = if args.len() > 1 {
        args[1..].join(" ")
    } else {
        default_user_message
    };

    let home_dir = dirs::home_dir().expect("Failed to get home directory");
    let config_path = home_dir.join(".rsii/config.toml");
    let config = fs::read_to_string(config_path).expect("Failed to read config file");
    let config: toml::Value = toml::from_str(&config).expect("Failed to parse config file");

    let model = config["default"]["model"]
        .as_str()
        .expect("Model not found in config file");

    let api_key = config["default"]["api-key"]
        .as_str()
        .expect("API key not found in config file");

    let system_prompt = config["default"]["system-prompt"]
        .as_str()
        .expect("System prompt not found in config file");

    let client = OpenAIClient::builder()
        .with_api_key(api_key)
        .build()
        .expect("Failed to create client");

    let user_arch = Command::new("uname")
        .arg("-a")
        .output()
        .expect("Failed to execute uname command");

    let user_arch_str = String::from_utf8_lossy(&user_arch.stdout);

    let total_prompt = format!(
        "{} Users system info: {} \n User query:\n{}",
        system_prompt, user_arch_str, user_message
    );

    let req = ChatCompletionRequest::new(
        model.to_string(),
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
        function: Function {
            name: "call_command".to_string(),
            description: Some("calls the given command for user".to_string()),
            parameters: FunctionParameters {
                schema_type: JSONSchemaType::Object,
                required: Some(vec!["command".to_string()]),
                properties: {
                    let mut map = std::collections::HashMap::new();

                    map.insert(
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

                    Some(map)
                },
            },
        },
    }]);
    println!("Getting AI response...");
    let result = client
        .chat_completion(req)
        .await
        .expect("Failed to get AI result");
    if let Some(tool_call) = &result.choices[0].message.tool_calls {
        for tool_call in tool_call {
            match tool_call {
                ToolCall {
                    id: _,
                    r#type: _,
                    function: ToolCallFunction { name, arguments },
                } => {
                    let tool_name = name.as_ref().unwrap();
                    let tool_arguments = arguments.as_ref().unwrap();
                    if tool_name == "call_command" {
                        let command = tool_arguments;
                        let command_value: Value =
                            serde_json::from_str(command).expect("Failed to parse command");
                        let command_str = command_value["command"]
                            .as_str()
                            .expect("Command not found in arguments");

                        let mut ctx: ClipboardContext =
                            ClipboardProvider::new().expect("Failed to create clipboard context");
                        ctx.set_contents(command_str.to_string())
                            .expect("Failed to set clipboard contents");

                        println!("Command is ready");

                        // Run a script to paste clipboard contents after 1 second
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
                }
            }
        }
    } else {
        println!("No tool calls found");
    }
}
