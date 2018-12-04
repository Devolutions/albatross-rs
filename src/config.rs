use std::fs;
use std::path::Path;
use std::error::Error;

use toml;

#[derive(Serialize,Deserialize)]
pub struct Config {
    pub output: String,
}

impl Config {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self, Box<Error>> {
        let contents = fs::read_to_string(path)?;
        Config::new_from_string(&contents)
    }

    pub fn new_from_string(contents: &str) -> Result<Self, Box<Error>> {
        let config: Config = toml::from_str(&contents)?;
        Ok(config)
    }
}

