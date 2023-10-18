use dirs;
use std::fs;
use toml::Table;

pub struct Config {
    pub public_gpg_key: String,
    pub base_directory: String,
}

pub fn get_config() -> Config {
    let mut config = Config {
        public_gpg_key: "".to_owned(),
        base_directory: "".to_owned(),
    };

    let home_dir: String = match dirs::home_dir() {
        Some(ref p) => p.to_string_lossy().to_string(),
        None => "".to_owned(),
    };

    if let Ok(config_string) = fs::read_to_string(home_dir + "/.config/npg/config.toml") {
        let val = config_string.parse::<Table>().unwrap();
        config.public_gpg_key = val["GPGPublicKey"].to_string();
        config.base_directory = val["BaseDirectory"].to_string();
    }

    return config;
}
