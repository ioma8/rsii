use std::fs;
use toml::Value;
use dirs;
use std::path::PathBuf;

pub struct Config {
    pub model: String,
    pub api_key: String,
    pub system_prompt: String,
}

pub fn load_config() -> Result<Config, Box<dyn std::error::Error>> {
    let home_dir: PathBuf = dirs::home_dir().ok_or("Failed to get home directory")?;
    let config_path = home_dir.join(".rsii/config.toml");
    let config_string = fs::read_to_string(config_path)?;
    let config: Value = toml::from_str(&config_string)?;

    Ok(Config {
        model: config["default"]["model"].as_str().ok_or("Model not found")?.to_string(),
        api_key: config["default"]["api-key"].as_str().ok_or("API key not found")?.to_string(),
        system_prompt: config["default"]["system-prompt"].as_str().ok_or("System prompt not found")?.to_string(),
    })
}
