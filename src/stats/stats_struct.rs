use std::{
    fs::{create_dir_all, write, File},
    path::PathBuf,
};

use dirs::config_dir;
use serde::{Deserialize, Serialize};
use termint::geometry::Coords;

use crate::error::Error;

use super::stat::Stat;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Stats {
    solves: Vec<Stat>,
    best: Option<Stat>,
}

impl Stats {
    /// Loads the stats with given board size
    pub fn load(size: &Coords) -> Self {
        match std::fs::read_to_string(Stats::get_file(size)) {
            Ok(s) => serde_json::from_str::<Self>(&s).unwrap_or_default(),
            Err(_) => Stats::default(),
        }
    }

    /// Saves stats wit
    pub fn save(&self, size: &Coords) -> Result<(), Error> {
        let mut path = Stats::get_dir()?;
        create_dir_all(&path)?;

        path.push(format!("{}x{}.json", size.x, size.y));
        File::create(&path)?;

        let text = serde_json::to_string::<Self>(self)?;
        write(path, text)?;
        Ok(())
    }

    /// Gets the solves
    pub fn solves(&self) -> &Vec<Stat> {
        &self.solves
    }

    /// Gets best solve of the stats
    pub fn best(&self) -> &Option<Stat> {
        &self.best
    }

    /// Adds given stat to the stats
    pub fn add(&mut self, stat: Stat) {
        match &self.best {
            Some(b) if stat.time() < b.time() => {
                self.best = Some(stat.clone())
            }
            None => self.best = Some(stat.clone()),
            _ => {}
        }
        self.solves.insert(0, stat);
    }

    /// Gets stats directory
    fn get_dir() -> Result<PathBuf, Error> {
        let mut config = config_dir()
            .ok_or(Error::Msg("Can't get stats directory".to_string()))?;
        config.push("loopover/stats");
        Ok(config)
    }

    /// Gets stats file
    fn get_file(size: &Coords) -> PathBuf {
        Stats::get_dir()
            .unwrap_or(PathBuf::from("."))
            .join(format!("{}x{}.json", size.x, size.y))
    }
}
