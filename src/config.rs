use std::{fs, path::Path};

use serde::Deserialize;

#[derive(Deserialize, Default, Clone)]
pub struct Config {
    pub rootpath: String,
    pub repos: Vec<String>,
}

pub fn read_config() -> Result<Config, String> {
    let toml = match fs::read_to_string("./config.toml") {
        Ok(toml) => toml,
        Err(_e) => return Err("Config file missing".to_owned()),
    };
    let config: Config = match toml::from_str(&toml) {
        Ok(c) => c,
        Err(_e) => return Err("Config file could not be parsed".to_owned()),
    };

    let root_path = Path::new(&config.rootpath);
    if !root_path.exists() {
        return Err("Root path in config doesn't exist".to_owned());
    }

    Ok(config)
}
