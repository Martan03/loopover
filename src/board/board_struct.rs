use rand::{seq::SliceRandom, Rng};
use termint::{geometry::Vec2, widgets::Widget};

use crate::error::Error;

/// Represents tictactoe board
#[derive(Debug, Clone)]
pub struct Board {
    pub cells: Vec<usize>,
    pub selected: Vec2,
    pub size: Vec2,
    pub small: bool,
}

impl Board {
    /// Creates new [`Board`]
    pub fn new(size: Vec2) -> Self {
        Self {
            cells: (1..=(size.x * size.y)).collect(),
            selected: Vec2::new(0, 0),
            size,
            small: size.x * size.y > 9,
        }
    }

    /// Checks if the [`Board`] is solved
    pub fn solved(&mut self) -> bool {
        for i in 0..(self.size.x * self.size.y - 1) {
            if self.cells[i] >= self.cells[i + 1] {
                return false;
            }
        }
        true
    }

    /// Scrambles the [`Board`]
    pub fn scramble(&mut self) {
        self.shuffle();
        while self.solved() {
            self.cells.shuffle(&mut rand::thread_rng());
        }
    }

    /// Restarts the game
    pub fn restart(&mut self) {
        self.cells = (1..=(self.size.x * self.size.y)).collect();
    }

    /// Sets selected cell
    pub fn select(&mut self, coords: Vec2) {
        self.selected = coords;
    }

    /// Moves selected up
    pub fn up(&mut self) {
        self.selected.y =
            self.selected.y.checked_sub(1).unwrap_or(self.size.y - 1);
    }

    /// Rotates selected column up
    pub fn move_up(&mut self) {
        self.rotate(self.selected.x, self.size.y, self.size.x as isize);
    }

    /// Moves selected up
    pub fn down(&mut self) {
        self.selected.y = (self.selected.y + 1) % self.size.y;
    }

    /// Rotates selected column down
    pub fn move_down(&mut self) {
        let start = self.selected.x + self.size.x * (self.size.y - 1);
        self.rotate(start, self.size.y, -(self.size.x as isize));
    }

    /// Moves selected up
    pub fn left(&mut self) {
        self.selected.x =
            self.selected.x.checked_sub(1).unwrap_or(self.size.x - 1);
    }

    /// Rotates selected row left
    pub fn move_left(&mut self) {
        let start = self.selected.y * self.size.x;
        self.rotate(start, self.size.x, 1);
    }

    /// Moves selected up
    pub fn right(&mut self) {
        self.selected.x = (self.selected.x + 1) % self.size.x;
    }

    /// Rotates selected row right
    pub fn move_right(&mut self) {
        let start = self.selected.y * self.size.x + self.size.x - 1;
        self.rotate(start, self.size.x, -1);
    }

    /// Applies reverse solution to the board to reconstruct the scramble
    pub fn apply_solution(
        &mut self,
        moves: &str,
        end: Vec2,
    ) -> Result<(), Error> {
        self.restart();
        self.select(end);

        for c in moves.chars().rev() {
            if c.is_whitespace() {
                continue;
            }
            self.apply_rev_move(c)?;
        }
        Ok(())
    }

    /// Applies given move
    pub fn apply_move(&mut self, m: char) -> Result<(), Error> {
        match m {
            'U' => {
                self.move_up();
                self.up();
            }
            'u' => self.up(),
            'R' => {
                self.move_right();
                self.right();
            }
            'r' => self.right(),
            'L' => {
                self.move_left();
                self.left();
            }
            'l' => self.left(),
            'D' => {
                self.move_down();
                self.down();
            }
            'd' => self.down(),
            _ => {
                return Err(Error::Msg(
                    "Invalid character in solution".to_string(),
                ))
            }
        }
        Ok(())
    }

    /// Removes given move
    pub fn apply_rev_move(&mut self, m: char) -> Result<(), Error> {
        match m {
            'U' => {
                self.move_down();
                self.down();
            }
            'u' => self.down(),
            'R' => {
                self.move_left();
                self.left();
            }
            'r' => self.left(),
            'L' => {
                self.move_right();
                self.right();
            }
            'l' => self.right(),
            'D' => {
                self.move_up();
                self.up();
            }
            'd' => self.up(),
            _ => {
                return Err(Error::Msg(
                    "Invalid character in solution".to_string(),
                ))
            }
        }
        Ok(())
    }
}

impl Board {
    /// Shuffles the [`Board`]
    fn shuffle(&mut self) {
        let mut rng = rand::thread_rng();
        let lim = self.size.x * self.size.x * self.size.y * self.size.y;
        for _ in 0..lim {
            let sel = Vec2::new(
                rng.gen_range(0..self.size.x),
                rng.gen_range(0..self.size.y),
            );

            match rng.gen_range(0..4) {
                0 => self.rotate(sel.x, self.size.y, self.size.x as isize),
                1 => {
                    let start = sel.x + self.size.x * (self.size.y - 1);
                    self.rotate(start, self.size.y, -(self.size.x as isize));
                }
                2 => self.rotate(sel.y * self.size.x, self.size.x, 1),
                3 => {
                    let start = sel.y * self.size.x + self.size.x - 1;
                    self.rotate(start, self.size.x, -1);
                }
                _ => {}
            }
        }
    }

    /// Applies rotation from the start position with given step
    fn rotate(&mut self, mut start: usize, size: usize, step: isize) {
        let cell = self.cells[start];
        for _ in 1..size {
            let next = (start as isize + step) as usize;
            self.cells[start] = self.cells[next];
            start = next;
        }
        self.cells[start] = cell;
    }
}

impl From<Board> for Box<dyn Widget> {
    fn from(value: Board) -> Self {
        Box::new(value)
    }
}
