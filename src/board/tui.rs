use termint::{
    buffer::Buffer,
    enums::{Color, Wrap},
    geometry::{Coords, Unit},
    widgets::{Grid, StrSpanExtension, Widget},
};

use crate::board::Board;

use super::asci::{get_cell, get_sel_cell};

impl Widget for Board {
    fn render(&self, buffer: &mut Buffer) {
        let mut grid = Grid::new(
            vec![Unit::Length(11); self.size.x],
            vec![Unit::Length(5); self.size.y],
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

    fn height(&self, _size: &Coords) -> usize {
        5 * self.size.y
    }

    fn width(&self, _size: &Coords) -> usize {
        11 * self.size.x
    }
}

impl Board {
    fn cell_color(&self, mut cell: usize) -> Color {
        cell -= 1;
        let x = (cell % self.size.x) * 200 / (self.size.x - 1);
        let y = (cell / self.size.x) * 200 / (self.size.y - 1);
        Color::Rgb((200 - x) as u8, y as u8, x as u8)
    }
}
