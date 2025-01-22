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

    // Generate version file
    let cargo_toml = fs::read_to_string("Cargo.toml").expect("Failed to read Cargo.toml");
    let cargo_toml: toml::Value = toml::from_str(&cargo_toml).expect("Failed to parse Cargo.toml");
    let version = cargo_toml["package"]["version"]
        .as_str()
        .expect("Version not found in Cargo.toml");

    let out_dir = std::env::var("OUT_DIR").expect("Failed to get OUT_DIR");
    let version_file_path = PathBuf::from(out_dir).join("version.rs");
    fs::write(version_file_path, format!("pub const VERSION: &str = \"{}\";", version))
        .expect("Failed to write version file");
}
