use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};
use std::{env, fs::File, path::PathBuf};

#[derive(Debug, Deserialize, Serialize)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub auth: AuthConfig,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AuthConfig {
    pub sk: String,
    pub pk: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub db_url: String,
    pub test_db_url: String,
    pub base_dir: PathBuf,
}

impl AppConfig {
    pub fn load() -> Result<Self> {
        // read from  ./chat.yaml, or /etc/config/chat.yaml, or env CHAT_CONFIG
        let ret = match (
            File::open("chat.yaml"),
            File::open("/etc/config/chat.yaml"),
            env::var("CHAT_CONFIG"),
        ) {
            (Ok(reader), _, _) => serde_yaml::from_reader(reader),
            (_, Ok(reader), _) => serde_yaml::from_reader(reader),
            (_, _, Ok(path)) => serde_yaml::from_reader(File::open(path)?),
            _ => bail!("Config file not found"),
        };
        Ok(ret?)
    }
}
