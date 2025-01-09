use std::{cell::RefCell, cmp::min, rc::Rc};

use termint::{
    buffer::Buffer,
    enums::{Color, Modifier},
    geometry::Vec2,
    style::Style,
    widgets::{Element, Widget},
};

use crate::{
    stats::{stat::Stat, stats_struct::Stats},
    tui::stats::StatsState,
};

#[derive(Debug)]
pub struct Table {
    stats: Stats,
    state: Rc<RefCell<StatsState>>,
}

impl Table {
    /// Creates new table widget
    pub fn new(stats: Stats, state: Rc<RefCell<StatsState>>) -> Self {
        Self { stats, state }
    }
}

impl Widget for Table {
    fn render(&self, buffer: &mut Buffer) {
        if buffer.height() == 0 || buffer.width() == 0 {
            return;
        }

        self.auto_scroll(buffer.height() - 1);
        self.render_scrollbar(buffer);
        self.render_header(buffer);
        let mut pos = *buffer.pos();
        pos.y += 1;

        if self.stats.solves().is_empty() {
            let style = Style::new().fg(Color::Gray);
            buffer.set_str_styled("Not stats yet...", &pos, style);
        }

        let selected = self.state.borrow().selected;
        for i in self.state.borrow().offset..self.stats.solves().len() {
            if buffer.y() + buffer.height() <= pos.y {
                break;
            }
            self.render_stat(buffer, &mut pos, &self.stats[i], i == selected);
            pos.y += 1;
        }
    }

    fn height(&self, size: &Vec2) -> usize {
        size.y
    }

    fn width(&self, _size: &Vec2) -> usize {
        35
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
        let time = stat.format_time();
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

    fn auto_scroll(&self, height: usize) {
        let mut state = self.state.borrow_mut();
        if state.selected < state.offset + 3 {
            state.offset = state.offset.saturating_sub(1);
        } else if state.selected + 3 >= state.offset + height {
            state.offset = std::cmp::min(
                state.offset + 1,
                self.stats.solves().len().saturating_sub(height),
            );
        }
    }

    fn calc_widths(width: usize) -> (usize, usize, usize) {
        let part = width.saturating_sub(1) as f64 / 20.;
        (
            (part * 9.) as usize,
            (part * 8.) as usize,
            (part * 5.) as usize,
        )
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
