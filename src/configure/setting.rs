use std::{collections::HashMap, sync::RwLock};

use config::*;
use tokio::io::Ready;
lazy_static::lazy_static! {
    static ref SETTINGS: RwLock<Config> = RwLock::new({
        let settings = Config::default();
        settings
    });
    static ref CONFIG_FILE_PATH: RwLock<String> = RwLock::new({
        let path = "".to_string();
        path
    });
}

pub fn set_config(path: &str) {
    if path.is_empty() {
        SETTINGS
            .write()
            .unwrap()
            .merge(File::with_name("settings.toml"))
            .unwrap();
    } else {
        SETTINGS
            .write()
            .unwrap()
            .merge(File::with_name(path))
            .unwrap();
    }
}

pub fn set_config_file_path(path: String) {
    CONFIG_FILE_PATH.write().unwrap().clear();
    CONFIG_FILE_PATH.write().unwrap().push_str(path.as_str());
}
pub fn get_config_file_path() -> String {
    CONFIG_FILE_PATH.read().unwrap().clone()
}

pub fn get_config() -> Result<HashMap<String, String>, ConfigError> {
    SETTINGS
        .read()
        .unwrap()
        .clone()
        .try_into::<HashMap<String, String>>()
}
