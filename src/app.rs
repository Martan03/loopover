use std::{
    io::{stdout, Write},
    time::Duration,
};

use crossterm::{
    event::{poll, read, Event, KeyCode, KeyEvent, KeyModifiers},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use termint::{
    enums::{Color, Modifier},
    geometry::{Constraint, Coords, TextAlign},
    term::Term,
    widgets::{Layout, Paragraph, Spacer, StrSpanExtension},
};

use crate::{board::Board, error::Error};

/// App struct containing the main loop, key listeners and rendering
#[derive(Debug)]
pub struct App {
    pub term: Term,
    pub board: Board,
    time: Duration,
}

impl App {
    /// Creates new [`App`] with board with given size and win length
    pub fn new(size: Coords) -> Self {
        Self {
            term: Term::new().small_screen(App::small_screen()),
            board: Board::new(size),
            time: Duration::from_secs(0),
        }
    }

    /// Runs the [`App`]
    pub fn run(&mut self) -> Result<(), Error> {
        // Saves screen, clears screen and hides cursor
        print!("\x1b[?1049h\x1b[2J\x1b[?25l");
        _ = stdout().flush();
        enable_raw_mode()?;

        let res = self.main_loop();

        disable_raw_mode()?;
        // Restores screen
        print!("\x1b[?1049l\x1b[?25h");
        _ = stdout().flush();

        match res {
            Err(Error::Exit) => Ok(()),
            _ => res,
        }
    }

    /// Main loop of the [`App`]
    fn main_loop(&mut self) -> Result<(), Error> {
        self.render()?;
        loop {
            if poll(Duration::from_millis(100))? {
                self.event()?;
            }
        }
    }

    /// Renders current screen of the [`App`]
    pub fn render(&mut self) -> Result<(), Error> {
        let mut board = Layout::horizontal();
        board.add_child(Spacer::new(), Constraint::Fill);
        board.add_child(self.board.clone(), Constraint::Min(0));
        board.add_child(self.simple_stats(), Constraint::Fill);

        let mut layout = Layout::vertical();
        layout.add_child(Spacer::new(), Constraint::Fill);
        layout.add_child(board, Constraint::Min(0));
        layout.add_child(Spacer::new(), Constraint::Fill);
        layout.add_child(App::render_help(), Constraint::Min(0));

        self.term.render(layout)?;
        Ok(())
    }

    /// Handles key listening
    fn event(&mut self) -> Result<(), Error> {
        match read()? {
            Event::Key(e) => self.key_handler(e),
            Event::Resize(_, _) => self.render(),
            _ => Ok(()),
        }
    }
}

impl App {
    /// Handles key events
    fn key_handler(&mut self, event: KeyEvent) -> Result<(), Error> {
        match event.code {
            KeyCode::Up => {
                if event.modifiers.contains(KeyModifiers::SHIFT) {
                    self.board.move_up();
                }
                self.board.up()
            }
            KeyCode::Down => {
                if event.modifiers.contains(KeyModifiers::SHIFT) {
                    self.board.move_down();
                }
                self.board.down();
            }
            KeyCode::Right => {
                if event.modifiers.contains(KeyModifiers::SHIFT) {
                    self.board.move_right();
                }
                self.board.right();
            }
            KeyCode::Left => {
                if event.modifiers.contains(KeyModifiers::SHIFT) {
                    self.board.move_left();
                }
                self.board.left();
            }
            KeyCode::Enter => self.board.scramble(),
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

    /// Small screen to be displayed, when game can't fit
    fn small_screen() -> Layout {
        let mut layout = Layout::vertical().center();
        layout.add_child(
            "Terminal too small!"
                .modifier(Modifier::BOLD)
                .align(TextAlign::Center),
            Constraint::Min(0),
        );
        layout.add_child(
            "You have to increase terminal size".align(TextAlign::Center),
            Constraint::Min(0),
        );
        layout
    }

    /// Gets simple stats layout
    fn simple_stats(&self) -> Layout {
        let mut layout = Layout::vertical().padding((0, 0, 0, 1));
        layout.add_child(
            format!("{:.3}", self.time.as_secs_f64()),
            Constraint::Min(0),
        );
        layout
    }

    /// Renders help with all the keybinds
    fn render_help() -> Paragraph {
        Paragraph::new(vec![
            "[Arrows]Move".fg(Color::Gray).into(),
            "[Shift+Arrow]Rotate".fg(Color::Gray).into(),
            "[Esc|q]Quit".fg(Color::Gray).into(),
        ])
        .separator("  ")
    }
}
