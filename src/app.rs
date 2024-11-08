use std::{
    io::{stdout, Write},
    time::{Duration, Instant},
};

use crossterm::{
    event::{poll, read, Event, KeyCode, KeyEvent, KeyModifiers},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use termint::{
    enums::{Color, Modifier},
    geometry::{Constraint, Coords, TextAlign},
    term::Term,
    widgets::{Layout, Paragraph, Spacer, StrSpanExtension, Widget},
};

use crate::{
    board::board_struct::Board,
    error::Error,
    stats::{stat::Stat, stats_struct::Stats},
};

#[derive(Debug, PartialEq, Eq)]
pub enum State {
    Scrambled,
    Playing,
    Idle,
}

/// App struct containing the main loop, key listeners and rendering
#[derive(Debug)]
pub struct App {
    term: Term,
    board: Board,
    time: Duration,
    moves_cnt: usize,
    moves: String,
    state: State,
    stats: Stats,
}

impl App {
    /// Creates new [`App`] with board with given size and win length
    pub fn new(size: Coords) -> Self {
        Self {
            term: Term::new().small_screen(App::small_screen()),
            board: Board::new(size),
            time: Duration::from_secs(0),
            moves_cnt: 0,
            moves: String::new(),
            state: State::Idle,
            stats: Stats::load(&size),
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

    fn game_loop(&mut self) -> Result<(), Error> {
        self.state = State::Playing;
        self.time = Duration::from_secs(0);
        self.moves_cnt = 1;
        self.render()?;

        let scramble = self.board.cells.clone();
        if self.board.solved() {
            return self.save_stat(scramble);
        }

        let start = Instant::now();
        let mut last = start;

        let mut running = true;
        while running {
            if poll(Duration::from_millis(1))? {
                self.time = start.elapsed();
                running = !self.event()?;
            } else if last.elapsed() >= Duration::from_secs_f64(0.001) {
                self.time = start.elapsed();
                last = Instant::now();
                self.render()?;
            }
        }

        self.save_stat(scramble)
    }

    /// Renders current screen of the [`App`]
    pub fn render(&mut self) -> Result<(), Error> {
        let mut board = Layout::horizontal();
        board.add_child(Spacer::new(), Constraint::Fill);
        board.add_child(self.board.clone(), Constraint::Min(0));
        board.add_child(self.simple_stats(), Constraint::Fill);

        let mut layout = Layout::vertical();
        layout.add_child(Spacer::new(), Constraint::Fill);
        layout.add_child(
            board,
            Constraint::Length(self.board.height(&Coords::new(0, 0))),
        );
        layout.add_child(Spacer::new(), Constraint::Fill);
        layout.add_child(App::render_help(), Constraint::Min(0));

        self.term.render(layout)?;
        Ok(())
    }

    /// Handles key listening
    fn event(&mut self) -> Result<bool, Error> {
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
        let solved = match event.code {
            KeyCode::Up => self.handle_move(
                |s| s.board.up(),
                |s| s.board.move_up(),
                'u',
                event,
            )?,
            KeyCode::Down => self.handle_move(
                |s| s.board.down(),
                |s| s.board.move_down(),
                'd',
                event,
            )?,
            KeyCode::Right => self.handle_move(
                |s| s.board.right(),
                |s| s.board.move_right(),
                'r',
                event,
            )?,
            KeyCode::Left => self.handle_move(
                |s| s.board.left(),
                |s| s.board.move_left(),
                'l',
                event,
            )?,
            KeyCode::Enter => {
                self.board.scramble();
                self.state = State::Scrambled;
                true
            }
            KeyCode::Char('c')
                if event.modifiers.contains(KeyModifiers::CONTROL) =>
            {
                return Err(Error::Exit);
            }
            KeyCode::Esc | KeyCode::Char('q') => return Err(Error::Exit),
            _ => return Ok(false),
        };
        self.render()?;
        Ok(solved)
    }

    fn handle_move<F1, F2>(
        &mut self,
        mov: F1,
        rot: F2,
        c: char,
        event: KeyEvent,
    ) -> Result<bool, Error>
    where
        F1: Fn(&mut App),
        F2: Fn(&mut App),
    {
        mov(self);
        if event.modifiers.contains(KeyModifiers::SHIFT) {
            rot(self);
            match self.state {
                State::Scrambled => {
                    self.moves = c.to_uppercase().to_string();
                    self.game_loop()?
                }
                State::Playing => {
                    self.moves_cnt += 1;
                    self.moves.push(' ');
                    self.moves.push(c.to_uppercase().next().unwrap_or(c))
                }
                _ => {}
            }
            return Ok(self.board.solved());
        } else if self.state == State::Playing {
            self.moves.push_str(&format!(" {c}"));
        }
        Ok(false)
    }

    /// Saves stat
    fn save_stat(&mut self, scramble: Vec<usize>) -> Result<(), Error> {
        if self.state == State::Playing {
            self.stats.add(Stat::new(
                self.time,
                self.moves_cnt,
                self.moves.clone(),
                scramble,
            ));
            self.stats.save(&self.board.size)?;
            self.state = State::Idle;
        }
        Ok(())
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
            format!("{:.3}", self.time.as_secs_f64())
                .fg(Color::White)
                .modifier(Modifier::BOLD),
            Constraint::Min(0),
        );

        self.simple_stats_moves(&mut layout);

        if let Some(best) = self.stats.best() {
            layout.add_child(self.simple_stats_best(best), Constraint::Min(0));
        }
        layout.add_child(Spacer::new(), Constraint::Length(1));

        self.simple_stats_list(&mut layout);
        layout
    }

    /// Gets moves count and moves per second
    fn simple_stats_moves(&self, layout: &mut Layout) {
        let mps = match self.time.as_secs_f64() {
            0.0 => 0.0,
            t => self.moves_cnt as f64 / t,
        };
        layout.add_child(
            format!("{} moves / {:.2} mps", self.moves_cnt, mps)
                .fg(Color::Gray),
            Constraint::Min(0),
        );
    }

    /// Gets the best time paragraph
    fn simple_stats_best(&self, best: &Stat) -> Paragraph {
        Paragraph::new(vec![
            format!("{:.3}", best.time().as_secs_f64())
                .fg(Color::Green)
                .into(),
            best.moves_cnt().to_string().fg(Color::DarkGreen).into(),
        ])
        .separator(" ")
    }

    /// Adds stats list to the simple stats layout
    fn simple_stats_list(&self, layout: &mut Layout) {
        let cnt = self.board.height(&Coords::new(0, 0)).saturating_sub(4);
        for stat in self.stats.solves().iter().take(cnt) {
            let p = Paragraph::new(vec![
                format!("{:.3}", stat.time().as_secs_f64())
                    .fg(Color::White)
                    .into(),
                stat.moves_cnt().to_string().fg(Color::Gray).into(),
            ])
            .separator(" ");
            layout.add_child(p, Constraint::Min(0));
        }
    }

    /// Renders help with all the keybinds
    fn render_help() -> Paragraph {
        Paragraph::new(vec![
            "[Arrows]Move".fg(Color::Gray).into(),
            "[Shift+Arrows]Rotate".fg(Color::Gray).into(),
            "[Enter]Scramble".fg(Color::Gray).into(),
            "[Esc|q]Quit".fg(Color::Gray).into(),
        ])
        .separator("  ")
    }
}
