use termint::{geometry::Coords, widgets::Widget};

mod asci;
mod tui;

/// Represents tictactoe board
#[derive(Debug, Clone)]
pub struct Board {
    pub cells: Vec<usize>,
    pub selected: Coords,
    pub size: Coords,
}

impl Board {
    /// Creates new [`Board`]
    pub fn new(size: Coords) -> Self {
        Self {
            cells: (1..=(size.x * size.y)).collect(),
            selected: Coords::new(0, 0),
            size,
        }
    }

    /// Restarts the game
    pub fn restart(&mut self) {
        self.cells = vec![0; self.size.x * self.size.y];
    }

    /// Sets selected cell
    pub fn select(&mut self, coords: Coords) {
        self.selected = coords;
    }

    /// Moves selected up
    pub fn up(&mut self) {
        self.selected.y =
            self.selected.y.checked_sub(1).unwrap_or(self.size.y - 1);
    }

    /// Rotates selected column up
    pub fn move_up(&mut self) {
        let cell = self.cells[self.selected.x];
        let mut id = self.selected.x;
        for _ in 1..self.size.y {
            self.cells[id] = self.cells[id + self.size.x];
            id += self.size.x;
        }
        self.cells[id] = cell;
    }

    /// Moves selected up
    pub fn down(&mut self) {
        self.selected.y = (self.selected.y + 1) % self.size.y;
    }

    /// Rotates selected column down
    pub fn move_down(&mut self) {
        let mut id = self.selected.x + self.size.x * (self.size.y - 1);
        let cell = self.cells[id];
        for _ in 1..self.size.y {
            self.cells[id] = self.cells[id - self.size.x];
            id -= self.size.x;
        }
        self.cells[id] = cell;
    }

    /// Moves selected up
    pub fn left(&mut self) {
        self.selected.x =
            self.selected.x.checked_sub(1).unwrap_or(self.size.x - 1);
    }

    /// Rotates selected row left
    pub fn move_left(&mut self) {
        let start = self.selected.y * self.size.x;
        let end = start + self.size.x;

        let first = self.cells[start];
        for i in start..end - 1 {
            self.cells[i] = self.cells[i + 1];
        }
        self.cells[end - 1] = first;
    }

    /// Moves selected up
    pub fn right(&mut self) {
        self.selected.x = (self.selected.x + 1) % self.size.x;
    }

    /// Rotates selected row right
    pub fn move_right(&mut self) {
        let start = self.selected.y * self.size.x;
        let end = start + self.size.x;

        let first = self.cells[end - 1];
        for i in (start + 1..end).rev() {
            self.cells[i] = self.cells[i - 1];
        }
        self.cells[start] = first;
    }
}

impl From<Board> for Box<dyn Widget> {
    fn from(value: Board) -> Self {
        Box::new(value)
    }
}
