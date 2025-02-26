use serde::{Deserialize, Serialize};
use std::{fs, sync::OnceLock};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub rdb_path: String,
    pub port: u32,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            rdb_path: "config.toml".to_string(),
            port: 7389,
        }
    }
}

pub fn get_config() -> &'static Config {
    static CONFIG: OnceLock<Config> = OnceLock::new();
    CONFIG.get_or_init(|| {
        let config_path = home::home_dir().unwrap().join(".config/mempaw/config.toml");
        if config_path.exists() {
            let config = fs::read_to_string(config_path).unwrap();
            toml::from_str(config.as_str()).unwrap()
        } else {
            Config::default()
        }
    })
}
