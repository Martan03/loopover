use std::time::Duration;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Stat {
    time: Duration,
    date: DateTime<Utc>,
    moves_cnt: usize,
    moves: String,
    scramble: Vec<usize>,
}

impl Stat {
    /// Creates new [`Stat`]
    pub fn new(
        time: Duration,
        moves_cnt: usize,
        moves: String,
        scramble: Vec<usize>,
    ) -> Self {
        Self {
            time,
            date: Utc::now(),
            moves_cnt,
            moves,
            scramble,
        }
    }

    /// Gets the time of the [`Stat`]
    pub fn time(&self) -> Duration {
        self.time
    }

    /// Gets the moves count
    pub fn moves_cnt(&self) -> usize {
        self.moves_cnt
    }

    /// Gets the scramblej of the [`Stat`]
    pub fn _scramble(&self) -> &Vec<usize> {
        &self.scramble
    }
}
