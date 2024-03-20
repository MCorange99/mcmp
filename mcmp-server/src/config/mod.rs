use std::path::PathBuf;
use anyhow::Result;
use serde::Deserialize;

use crate::cli::CliArgs;

const DEFAULT_CONFIG: &'static str = include_str!("../../default_config.toml");

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub database: ConfigDatabase,
    pub port: u16
}

#[derive(Debug, Deserialize, Clone)]
pub struct ConfigDatabase {
    pub uri: String,
}



impl Config {
    pub fn parse(path: PathBuf) -> Result<Self> {

        let data = if !path.exists() {
            std::fs::write(&path, DEFAULT_CONFIG)?;
            DEFAULT_CONFIG.to_string()
        } else {
            std::fs::read_to_string(&path)?
        };

        let slf = toml::from_str(data.as_str())?;
        Ok(slf)
    }

    pub fn append(&mut self, cli: CliArgs) {
        if let Some(port) = cli.port {
            self.port = port;
        }
    }
}
