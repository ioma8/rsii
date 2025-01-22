use std::fs;
use std::path::PathBuf;

fn main() {
    let home_dir = dirs::home_dir().expect("Failed to get home directory");
    let config_path = home_dir.join(".rsii/config.toml");
    if !config_path.exists() {
        fs::create_dir_all(config_path.parent().expect("Failed to get parent directory")).expect("Failed to create .rsii directory");
        let default_config_path = PathBuf::from("default.config.toml");
        fs::copy(default_config_path, config_path).expect("Failed to copy default.config.toml to home directory");
    }
}
