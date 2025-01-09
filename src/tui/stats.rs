use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use termint::{geometry::Constraint, widgets::Layout};

use crate::{
    app::{App, Screen},
    error::Error,
};

use super::widget::table::Table;

//===========================================================================//
//                          Public stats methods                             //
//===========================================================================//
impl App {
    /// Renders the game screen
    pub fn render_stats(&mut self) -> Result<(), Error> {
        let mut layout = Layout::vertical();
        layout.push(
            Table::new(self.stats.clone(), self.stat_state.clone()),
            Constraint::Fill(1),
        );

        self.term.render(layout)?;
        Ok(())
    }

    /// Handles key events for the game screen
    pub fn listen_stats(&mut self, event: KeyEvent) -> Result<(), Error> {
        match event.code {
            KeyCode::Up => self.select_prev(),
            KeyCode::Down => self.select_next(),
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
}

//===========================================================================//
//                          Private stats methods                            //
//===========================================================================//
impl App {
    fn select_next(&mut self) {
        let mut state = self.stat_state.borrow_mut();

        if state.selected + 1 < self.stats.solves().len() {
            state.selected = state.selected + 1;
        }
    }

    /// Sets selected mode to the previous value if previous exists
    fn select_prev(&mut self) {
        let mut state = self.stat_state.borrow_mut();
        state.selected = state.selected.saturating_sub(1);
    }
}
