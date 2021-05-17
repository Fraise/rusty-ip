// use std::fs::File;
use std::fs;
use std::error::Error;
use std::io::Read;
use toml;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub listen_address: String,
    pub listen_port: u32,
}

pub fn parse(path: &str) -> Result<Config, Box<dyn Error>> {
    let config: Config = toml::from_str(fs::read_to_string(path)?.as_str())?;

    Ok(config)
}