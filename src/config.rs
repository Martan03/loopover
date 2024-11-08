use std::{
    fs::File,
    io::{BufReader, BufWriter},
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};

use crate::{error::Error, size::Size};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub default_size: Size,
}

impl Config {
    /// Loads config from default json file path
    pub fn load() -> Self {
        Self::from_json(Self::get_path()).unwrap_or_default()
    }

    /// Saves config to default json path
    pub fn save(&self) -> Result<(), Error> {
        self.to_json(Self::get_path())
    }

    /// Loads config from given path
    pub fn from_json(file: impl AsRef<Path>) -> Result<Self, Error> {
        let buffer = BufReader::new(File::open(file)?);
        Ok(serde_json::from_reader(buffer)?)
    }

    /// Saves config to given path
    pub fn to_json(&self, file: impl AsRef<Path>) -> Result<(), Error> {
        let buffer = BufWriter::new(File::create(file)?);
        Ok(serde_json::to_writer_pretty(buffer, self)?)
    }

    /// Gets config directory
    pub fn get_dir() -> PathBuf {
        dirs::config_dir().unwrap_or(".".into()).join("loopover")
    }

    /// Gets config file path
    pub fn get_path() -> PathBuf {
        Self::get_dir().join("config.json")
    }
}
