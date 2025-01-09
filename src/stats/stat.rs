use std::time::Duration;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use termint::geometry::Vec2;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Stat {
    time: Duration,
    date: DateTime<Utc>,
    moves_cnt: usize,
    moves: String,
    end_x: usize,
    end_y: usize,
}

impl Stat {
    /// Creates new [`Stat`]
    pub fn new(
        time: Duration,
        moves_cnt: usize,
        moves: String,
        end: Vec2,
    ) -> Self {
        Self {
            time,
            date: Utc::now(),
            moves_cnt,
            moves,
            end_x: end.x,
            end_y: end.y,
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

    /// Gets the moves used to solve the scramble
    pub fn moves(&self) -> &String {
        &self.moves
    }

    /// Gets the end position of the cursor
    pub fn end(&self) -> Vec2 {
        Vec2::new(self.end_x, self.end_y)
    }

    /// Gets the date of the solve
    pub fn date(&self) -> DateTime<Utc> {
        self.date
    }

    pub fn format_time(&self) -> String {
        let total = self.time.as_millis();
        let mins = total / 60000;
        let secs = (total / 1000) % 60;
        let millis = total % 1000;

        if mins > 0 {
            format!("{}:{:02}.{:03}", mins, secs, millis)
        } else {
            format!("{:02}.{:03}", secs, millis)
        }
    }
}
