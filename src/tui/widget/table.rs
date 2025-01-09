use std::{cell::RefCell, cmp::min, rc::Rc, time::Duration};

use termint::{
    buffer::Buffer,
    enums::{Color, Modifier},
    geometry::Vec2,
    style::Style,
    widgets::{Element, Widget},
};

use crate::stats::{stat::Stat, stats_struct::Stats};

#[derive(Debug)]
pub struct TableState {
    pub offset: usize,
    pub selected: usize,
}

impl TableState {
    pub fn new() -> Self {
        Self {
            offset: 0,
            selected: 0,
        }
    }
}

#[derive(Debug)]
pub struct Table {
    stats: Stats,
    state: Rc<RefCell<TableState>>,
}

impl Table {
    /// Creates new table widget
    pub fn new(stats: Stats, state: Rc<RefCell<TableState>>) -> Self {
        Self { stats, state }
    }
}

impl Widget for Table {
    fn render(&self, buffer: &mut Buffer) {
        if buffer.height() == 0 || buffer.width() == 0 {
            return;
        }

        let mut pos = *buffer.pos();
        self.render_scrollbar(buffer);
        self.render_header(buffer);

        let selected = self.state.borrow().selected;
        for i in self.state.borrow().offset..self.stats.solves().len() {
            pos.y += 1;
            if buffer.y() + buffer.height() <= pos.y {
                break;
            }
            self.render_stat(buffer, &mut pos, &self.stats[i], i == selected);
        }
    }

    fn height(&self, size: &Vec2) -> usize {
        size.y
    }

    fn width(&self, size: &Vec2) -> usize {
        25
    }
}

impl Table {
    /// Renders scrollbar
    fn render_scrollbar(&self, buffer: &mut Buffer) {
        let rat = self.stats.solves().len() as f32 / buffer.height() as f32;
        let thumb_size = min(
            (buffer.height() as f32 / rat).floor() as usize,
            buffer.height(),
        );
        let thumb_offset = min(
            (self.state.borrow().offset as f32 / rat) as usize,
            buffer.height() - thumb_size,
        );

        let x = (buffer.x() + buffer.width()).saturating_sub(1);
        let mut bar_pos = Vec2::new(x, buffer.y());
        for _ in 0..buffer.height() {
            buffer.set_val('│', &bar_pos);
            bar_pos.y += 1;
        }

        bar_pos = Vec2::new(x, buffer.y() + thumb_offset);
        for _ in 0..thumb_size {
            buffer.set_val('┃', &bar_pos);
            bar_pos.y += 1;
        }
    }

    fn render_header(&self, buffer: &mut Buffer) {
        let (dwidth, twidth, mwidth) = Self::calc_widths(buffer.width());
        let mut pos = *buffer.pos();

        let style = Style::new().fg(Color::White).modifier(Modifier::BOLD);
        buffer.set_str_styled(
            "Date".chars().take(dwidth).collect::<String>(),
            &pos,
            style,
        );
        pos.x += dwidth;
        buffer.set_str_styled(
            "Time".chars().take(twidth).collect::<String>(),
            &pos,
            style,
        );
        pos.x += twidth;
        buffer.set_str_styled(
            "Moves".chars().take(mwidth).collect::<String>(),
            &pos,
            style,
        );
    }

    /// Renders given stat into the table
    fn render_stat(
        &self,
        buffer: &mut Buffer,
        pos: &mut Vec2,
        stat: &Stat,
        selected: bool,
    ) {
        let (dwidth, twidth, mwidth) = Self::calc_widths(buffer.width());

        let date = stat.date().format("%d/%m/%Y").to_string();
        let time = Self::format_duration(stat.time());
        let move_cnt = stat.moves_cnt().to_string();

        let style = match selected {
            true => Style::new().fg(Color::Cyan),
            false => Style::new().fg(Color::Gray),
        };

        buffer.set_str_styled(
            date.chars().take(dwidth).collect::<String>(),
            pos,
            style,
        );
        pos.x += dwidth;
        buffer.set_str_styled(
            time.chars().take(twidth).collect::<String>(),
            pos,
            style,
        );
        pos.x += twidth;
        buffer.set_str_styled(
            move_cnt.chars().take(mwidth).collect::<String>(),
            pos,
            style,
        );
        pos.x = buffer.x();
    }

    fn calc_widths(width: usize) -> (usize, usize, usize) {
        let part = width.saturating_sub(1) as f64 / 20.;
        (
            (part * 10.) as usize,
            (part * 7.) as usize,
            (part * 3.) as usize,
        )
    }

    fn format_duration(duration: Duration) -> String {
        let total = duration.as_millis();
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

impl From<Table> for Element {
    fn from(value: Table) -> Self {
        Element::new(value)
    }
}

impl From<Table> for Box<dyn Widget> {
    fn from(value: Table) -> Self {
        Box::new(value)
    }
}
