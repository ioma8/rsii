use std::fs;
use std::path::PathBuf;

use dirs;
use serde::Deserialize;

pub struct Config {
    pub model: String,
    pub api_key: String,
    pub system_prompt: String,
}

#[derive(Deserialize)]
struct FileConfig {
    default: DefaultSection,
}

#[derive(Deserialize)]
struct DefaultSection {
    model: String,
    #[serde(rename = "api-key")]
    api_key: String,
    #[serde(rename = "system-prompt")]
    system_prompt: String,
}

pub fn load_config() -> Result<Config, Box<dyn std::error::Error>> {
    let home_dir: PathBuf = dirs::home_dir().ok_or_else(|| {
        "Failed to determine user home directory".to_string()
    })?;
    let config_path = home_dir.join(".rsii/config.toml");
    if !config_path.exists() {
        return Err(format!(
            "Config file not found at {}. Build should have created it.",
            config_path.display()
        )
        .into());
    }

    let config_string = fs::read_to_string(&config_path).map_err(|e| {
        format!(
            "Failed to read config at {}: {}",
            config_path.display(),
            e
        )
    })?;

    let file_conf: FileConfig = toml::from_str(&config_string).map_err(|e| {
        format!(
            "Failed to parse TOML at {}: {}",
            config_path.display(),
            e
        )
    })?;

    Ok(Config {
        model: file_conf.default.model,
        api_key: file_conf.default.api_key,
        system_prompt: file_conf.default.system_prompt,
    })
}
