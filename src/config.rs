use std::fmt;
use std::fs;
use std::error::Error;
use toml;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub listen_address: String,
    pub listen_port: u32,
    pub path: String,
}

impl fmt::Display for Config {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s: String = format!( "listen_address: {}\n", self.listen_address);
        s.push_str(format!( "listen_port: {}\n", self.listen_port).as_str());
        s.push_str(format!( "path: {}\n", self.path).as_str());

        f.write_str(s.as_str())
    }
}

pub fn default() -> Config {
    Config {
        listen_port: 80,
        listen_address: String::from("0.0.0.0"),
        path: String::from("/"),
    }
}

pub fn parse(path: &str) -> Result<Config, Box<dyn Error>> {
    let config: Config = toml::from_str(fs::read_to_string(path)?.as_str())?;

    Ok(config)
}