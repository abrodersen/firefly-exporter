
use std::fs::OpenOptions;
use std::io::Read;

use serde::{Serialize, Deserialize};

use toml::from_slice;

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub firefly: Firefly,
}

#[derive(Serialize, Deserialize)]
pub struct Firefly {
    pub url: String,
    pub token: String
}

pub fn read_config() -> Result<Config, Box<dyn std::error::Error>> {
    let dirs = xdg::BaseDirectories::with_prefix("firefly-editor")?;
    let cfg_path = dirs.place_config_file("config.toml")?;
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(cfg_path)?;

    let mut buf = Vec::new();
    file.read_to_end(&mut buf)?;
    let cfg = from_slice(&buf)?;
    Ok(cfg)
}