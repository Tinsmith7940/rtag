use dirs::config_local_dir;
use serde::Deserialize;
use std::fs;
use toml::{self, map::Map};

#[derive(Deserialize, Debug)]
pub struct Config {
    #[allow(dead_code)]
    pub profile: Map<String, toml::Value>,
    pub clear: Option<bool>,
}

pub fn get_config() -> Option<Config> {
    if let Some(local_config) = config_local_dir() {
        let path = local_config.to_str().unwrap().to_owned() + "/audiotag-ci/config.toml";
        if let Ok(file_contents) = fs::read_to_string(path) {
            match toml::from_str(&file_contents) {
                Ok(cnf) => {
                    return Some(cnf);
                }
                Err(e) => {
                    log::warn!("Unable to successfully parse toml config: {e}");
                    return None;
                }
            }
        }
    }

    None
}
