use crate::configure::config_error::{ConfigError, ConfigErrorType};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_yaml::from_str;
use std::fs;
use std::path::Path;
use std::sync::Mutex;
use std::sync::RwLock;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Config {
    pub server: String,
    pub token: String,
}

impl Config {
    pub fn default() -> Self {
        Self {
            server: "http://127.0.0.1:8080".to_string(),
            token: "".to_string(),
        }
    }

    pub fn set_self(&mut self, config: Config) {
        self.server = config.server;
        self.token = config.token;
    }

    pub fn get_config_image(&self) -> Self {
        self.clone()
    }

    pub fn flush_to_file(&self, path: String) -> Result<()> {
        let yml = serde_yaml::to_string(&self)?;
        fs::write(path, yml)?;
        Ok(())
    }
}

pub fn generate_default_config(path: &str) -> Result<()> {
    let config = Config::default();
    let yml = serde_yaml::to_string(&config)?;
    fs::write(path, yml)?;
    Ok(())
}

lazy_static::lazy_static! {
    static ref GLOBAL_CONFIG: Mutex<Config> = {
        let global_config = Config::default();
        Mutex::new(global_config)
    };
    static ref CONFIG_FILE_PATH: RwLock<String> = RwLock::new({
        let path = "".to_string();
        path
    });
}

pub fn set_config(config: Config) {
    GLOBAL_CONFIG.lock().unwrap().set_self(config);
}

pub fn set_config_from_file(path: &str) {
    if path.is_empty() {
        if Path::new("config.yml").exists() {
            let contents =
                fs::read_to_string("config.yml").expect("Read config file config.yml error!");
            let config = from_str::<Config>(contents.as_str()).expect("Parse config.yml error!");
            GLOBAL_CONFIG.lock().unwrap().set_self(config);
            set_config_file_path("./config.yml".to_string());
        }
        return;
    }

    let err_str = format!("Read config file {} error!", path);
    let contents = fs::read_to_string(path).expect(err_str.as_str());
    let config = from_str::<Config>(contents.as_str()).expect("Parse config.yml error!");
    GLOBAL_CONFIG.lock().unwrap().set_self(config);
    set_config_file_path(path.to_string());
}

pub fn set_config_file_path(path: String) {
    CONFIG_FILE_PATH
        .write()
        .expect("clear config file path error!")
        .clear();
    CONFIG_FILE_PATH.write().unwrap().push_str(path.as_str());
}

pub fn get_config_file_path() -> String {
    CONFIG_FILE_PATH.read().unwrap().clone()
}

pub fn get_config() -> Result<Config> {
    let locked_config = GLOBAL_CONFIG.lock().map_err(|e| {
        return ConfigError::from_err(e.to_string(), ConfigErrorType::UnknowErr);
    })?;
    Ok(locked_config.get_config_image())
}

pub fn get_current_config_yml() -> Result<String> {
    let c = get_config()?;
    let yml = serde_yaml::to_string(&c)?;
    Ok(yml)
}
