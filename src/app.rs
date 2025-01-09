use std::{
    cell::RefCell,
    io::{stdout, Write},
    rc::Rc,
    time::Duration,
};

use crossterm::{
    event::{poll, read, Event, KeyEvent},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use termint::{
    enums::Modifier,
    geometry::{Constraint, TextAlign, Vec2},
    term::Term,
    widgets::{Layout, StrSpanExtension},
};

use crate::{
    board::board_struct::Board, error::Error, stats::stats_struct::Stats,
    tui::widget::table::TableState,
};

#[derive(Debug, PartialEq, Eq)]
pub enum Screen {
    Game,
    Stats,
}

#[derive(Debug, PartialEq, Eq)]
pub enum State {
    Scrambled,
    Playing,
    Idle,
}

/// App struct containing the main loop, key listeners and rendering
#[derive(Debug)]
pub struct App {
    pub term: Term,
    pub board: Board,
    pub time: Duration,
    pub moves_cnt: usize,
    pub start_pos: Vec2,
    pub moves: String,
    pub screen: Screen,
    pub state: State,
    pub stats: Stats,
    pub stat_state: Rc<RefCell<TableState>>,
}

impl App {
    /// Creates new [`App`] with board with given size and win length
    pub fn new(size: Vec2) -> Self {
        Self {
            term: Term::new().small_screen(App::small_screen()),
            board: Board::new(size),
            time: Duration::from_secs(0),
            moves_cnt: 0,
            start_pos: Vec2::new(0, 0),
            moves: String::new(),
            screen: Screen::Game,
            state: State::Idle,
            stats: Stats::load(&size),
            stat_state: Rc::new(RefCell::new(TableState::new())),
        }
    }

    /// Runs the [`App`]
    pub fn run(&mut self) -> Result<(), Error> {
        enable_raw_mode()?;
        // Saves screen, clears screen and hides cursor
        print!("\x1b[?1049h\x1b[2J\x1b[?25l");
        _ = stdout().flush();

        let res = self.main_loop();

        // Restores screen
        print!("\x1b[?1049l\x1b[?25h");
        _ = stdout().flush();
        disable_raw_mode()?;

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
        match self.screen {
            Screen::Game => self.render_game(),
            Screen::Stats => self.render_stats(),
        }
    }

    /// Handles key listening
    pub fn event(&mut self) -> Result<bool, Error> {
        match read()? {
            Event::Key(e) => self.key_handler(e),
            Event::Resize(_, _) => {
                self.render()?;
                Ok(false)
            }
            _ => Ok(false),
        }
    }
}

impl App {
    /// Handles key events
    fn key_handler(&mut self, event: KeyEvent) -> Result<bool, Error> {
        match self.screen {
            Screen::Game => self.listen_game(event),
            Screen::Stats => {
                self.listen_stats(event)?;
                Ok(false)
            }
        }
    }

    /// Small screen to be displayed, when game can't fit
    fn small_screen() -> Layout {
        let mut layout = Layout::vertical().center();
        layout.push(
            "Terminal too small!"
                .modifier(Modifier::BOLD)
                .align(TextAlign::Center),
            Constraint::Min(0),
        );
        layout.push(
            "You have to increase terminal size".align(TextAlign::Center),
            Constraint::Min(0),
        );
        layout
    }
}
