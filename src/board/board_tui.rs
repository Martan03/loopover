use termint::{
    buffer::Buffer,
    enums::{Color, Wrap},
    geometry::{Coords, Unit},
    widgets::{Grid, StrSpanExtension, Widget},
};

use super::{
    asci::{get_cell, get_min_cell, get_min_sel_cell, get_sel_cell},
    board_struct::Board,
};

impl Widget for Board {
    fn render(&self, buffer: &mut Buffer) {
        match self.small {
            true => self._render(
                buffer,
                (7, 3),
                |n| get_min_cell(n),
                |n| get_min_sel_cell(n),
            ),
            false => self._render(
                buffer,
                (11, 5),
                |n| get_cell(n),
                |n| get_sel_cell(n),
            ),
        }
    }

    fn height(&self, _size: &Coords) -> usize {
        match self.small {
            true => 3 * self.size.y,
            false => 5 * self.size.y,
        }
    }

    fn width(&self, _size: &Coords) -> usize {
        match self.small {
            true => 7 * self.size.x,
            false => 11 * self.size.x,
        }
    }
}

impl Board {
    fn _render<F1, F2>(
        &self,
        buffer: &mut Buffer,
        (x, y): (usize, usize),
        get_cell: F1,
        get_sel_cell: F2,
    ) where
        F1: Fn(usize) -> String,
        F2: Fn(usize) -> String,
    {
        let mut grid = Grid::new(
            vec![Unit::Length(x); self.size.x],
            vec![Unit::Length(y); self.size.y],
        );

        for y in 0..self.size.y {
            for x in 0..self.size.x {
                let val = self.cells[x + y * self.size.x];
                let cell = if self.selected.x == x && self.selected.y == y {
                    get_sel_cell(val)
                } else {
                    get_cell(val)
                };

                let span = cell
                    .wrap(Wrap::Letter)
                    .bg(self.cell_color(val))
                    .fg(Color::White);
                grid.add_child(span, x, y);
            }
        }

        grid.render(buffer);
    }

    fn cell_color(&self, mut cell: usize) -> Color {
        cell -= 1;
        let x = (cell % self.size.x) * 200 / (self.size.x - 1);
        let y = (cell / self.size.x) * 200 / (self.size.y - 1);
        Color::Rgb((200 - x) as u8, y as u8, x as u8)
    }
}
