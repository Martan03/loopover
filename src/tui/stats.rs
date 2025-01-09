use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use termint::{
    enums::Color,
    geometry::Constraint,
    style::Style,
    widgets::{Block, BorderType, Layout, Spacer, StrSpanExtension},
};

use crate::{
    app::{App, Screen},
    error::Error,
};

use super::widget::table::Table;

/// Stats state containing list offset, selected stat and move offset
#[derive(Debug, Default)]
pub struct StatsState {
    pub offset: usize,
    pub selected: usize,
    pub move_offset: usize,
}

//===========================================================================//
//                          Public stats methods                             //
//===========================================================================//
impl App {
    /// Renders the game screen
    pub fn render_stats(&mut self) -> Result<(), Error> {
        let mut layout = Block::horizontal().border_type(BorderType::Thicker);
        layout.push(
            Table::new(self.stats.clone(), self.stat_state.clone()),
            Constraint::Min(1),
        );
        self.render_stat(&mut layout);

        let mut hor_center = Layout::horizontal().center();
        hor_center.push(layout, Constraint::Min(0));
        let mut ver_center = Layout::vertical().center();
        ver_center.push(hor_center, Constraint::Percent(75));
        self.term.render(ver_center)?;
        Ok(())
    }

    /// Handles key events for the game screen
    pub fn listen_stats(&mut self, event: KeyEvent) -> Result<(), Error> {
        match event.code {
            KeyCode::Up => {
                self.select_prev();
                self.load_stat_board()?;
            }
            KeyCode::Down => {
                self.select_next();
                self.load_stat_board()?;
            }
            KeyCode::Left => self.prev_move()?,
            KeyCode::Right => self.next_move()?,
            KeyCode::Tab => self.screen = Screen::Game,
            KeyCode::Char('c')
                if event.modifiers.contains(KeyModifiers::CONTROL) =>
            {
                return Err(Error::Exit);
            }
            KeyCode::Esc | KeyCode::Char('q') => return Err(Error::Exit),
            _ => return Ok(()),
        }
        self.render()
    }

    /// Loads the board of currently selected
    pub fn load_stat_board(&mut self) -> Result<(), Error> {
        let state = self.stat_state.borrow();
        if let Some(stat) = self.stats.solves().get(state.selected) {
            self.stat_board.apply_solution(stat.moves(), stat.end())?;
        }
        Ok(())
    }
}

//===========================================================================//
//                          Private stats methods                            //
//===========================================================================//
impl App {
    fn render_stat(&self, layout: &mut Block<Layout>) {
        let state = self.stat_state.borrow();
        let Some(stat) = self.stats.solves().get(state.selected) else {
            return;
        };

        let mut slayout = Layout::vertical().padding((0, 1));
        Self::render_item(
            &mut slayout,
            "Time:",
            &stat.format_time(),
            Style::new().fg(Color::Cyan),
        );
        Self::render_item(
            &mut slayout,
            "Date:",
            &stat.date().format("%d/%m/%Y %H:%M:%S").to_string(),
            Style::new().fg(Color::DarkYellow),
        );
        Self::render_item(
            &mut slayout,
            "Moves:",
            &stat.moves_cnt().to_string(),
            Style::new().fg(Color::Red),
        );
        slayout.push(Spacer::new(), Constraint::Fill(1));

        let mut wrapper = Layout::horizontal().center();
        wrapper.push(self.stat_board.clone(), Constraint::Min(0));

        slayout.push(wrapper, Constraint::Min(0));
        slayout.push(Spacer::new(), Constraint::Fill(1));
        layout.push(slayout, Constraint::Min(0));
    }

    fn render_item(
        layout: &mut Layout,
        key: &str,
        value: &str,
        val_style: Style,
    ) {
        let mut wrapper = Layout::horizontal();
        wrapper.push(key.fg(Color::White), Constraint::Length(7));
        wrapper.push(value.style(val_style), Constraint::Min(1));
        layout.push(wrapper, Constraint::Length(1));
    }

    fn select_next(&mut self) {
        let mut state = self.stat_state.borrow_mut();
        state.move_offset = 0;

        if state.selected + 1 < self.stats.solves().len() {
            state.selected += 1;
        }
    }

    /// Sets selected mode to the previous value if previous exists
    fn select_prev(&mut self) {
        let mut state = self.stat_state.borrow_mut();
        state.move_offset = 0;
        state.selected = state.selected.saturating_sub(1);
    }

    /// Applies next move to the current solve preview
    fn next_move(&mut self) -> Result<(), Error> {
        let mut state = self.stat_state.borrow_mut();
        let stat = &self.stats[state.selected];
        if state.move_offset >= stat.moves().len() {
            return Ok(());
        }

        for c in stat.moves().chars().skip(state.move_offset) {
            state.move_offset += 1;
            if c.is_whitespace() {
                continue;
            }
            self.stat_board.apply_move(c)?;
            return Ok(());
        }
        state.move_offset -= 1;
        Ok(())
    }

    /// Applies inverse prev move to the current solve preview
    fn prev_move(&mut self) -> Result<(), Error> {
        let mut state = self.stat_state.borrow_mut();
        let stat = &self.stats[state.selected];
        if state.move_offset == 0 {
            return Ok(());
        }

        let offset = stat.moves().len().saturating_sub(state.move_offset);
        for c in stat.moves().chars().rev().skip(offset) {
            state.move_offset = state.move_offset.saturating_sub(1);
            if c.is_whitespace() {
                continue;
            }
            self.stat_board.apply_rev_move(c)?;
            break;
        }
        Ok(())
    }
}
