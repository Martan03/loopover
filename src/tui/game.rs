use std::time::{Duration, Instant};

use crossterm::event::{poll, KeyCode, KeyEvent, KeyModifiers};
use termint::{
    enums::{Color, Modifier},
    geometry::{Constraint, Vec2},
    widgets::{Layout, Paragraph, Spacer, StrSpanExtension, Widget},
};

use crate::{
    app::{App, Screen, State},
    error::Error,
    stats::stat::Stat,
};

//===========================================================================//
//                           Public game methods                             //
//===========================================================================//
impl App {
    /// Renders the game screen
    pub fn render_game(&mut self) -> Result<(), Error> {
        let mut board = Layout::horizontal();
        board.push(Spacer::new(), Constraint::Fill(1));
        board.push(self.board.clone(), Constraint::Min(0));
        board.push(self.simple_stats(), Constraint::Fill(1));

        let mut layout = Layout::vertical();
        layout.push(Spacer::new(), Constraint::Fill(1));
        layout.push(
            board,
            Constraint::Length(self.board.height(&Vec2::new(0, 0))),
        );
        layout.push(Spacer::new(), Constraint::Fill(1));
        layout.push(App::render_help(), Constraint::Min(0));

        self.term.render(layout)?;
        Ok(())
    }

    /// Handles key events for the game screen
    pub fn listen_game(&mut self, event: KeyEvent) -> Result<bool, Error> {
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
            KeyCode::Tab => {
                self.screen = Screen::Stats;
                false
            }
            KeyCode::Esc | KeyCode::Char('q') => return Err(Error::Exit),
            _ => return Ok(false),
        };
        self.render()?;
        Ok(solved)
    }
}

//===========================================================================//
//                        Rendering helper methods                           //
//===========================================================================//
impl App {
    /// Gets simple stats layout
    fn simple_stats(&self) -> Layout {
        let mut layout = Layout::vertical().padding((0, 0, 0, 1));
        layout.push(
            format!("{:.3}", self.time.as_secs_f64())
                .fg(Color::White)
                .modifier(Modifier::BOLD),
            Constraint::Min(0),
        );

        self.simple_stats_moves(&mut layout);

        if let Some(best) = self.stats.best() {
            layout.push(self.simple_stats_best(best), Constraint::Min(0));
        }
        layout.push(Spacer::new(), Constraint::Length(1));

        self.simple_stats_list(&mut layout);
        layout
    }

    /// Gets moves count and moves per second
    fn simple_stats_moves(&self, layout: &mut Layout) {
        let mps = match self.time.as_secs_f64() {
            0.0 => 0.0,
            t => self.moves_cnt as f64 / t,
        };
        layout.push(
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
        let cnt = self.board.height(&Vec2::new(0, 0)).saturating_sub(4);
        for stat in self.stats.solves().iter().take(cnt) {
            let p = Paragraph::new(vec![
                format!("{:.3}", stat.time().as_secs_f64())
                    .fg(Color::White)
                    .into(),
                stat.moves_cnt().to_string().fg(Color::Gray).into(),
            ])
            .separator(" ");
            layout.push(p, Constraint::Min(0));
        }
    }

    /// Renders help with all the keybinds
    fn render_help() -> Paragraph {
        Paragraph::new(vec![
            "[Arrows]Move".fg(Color::Gray).into(),
            "[Shift+Arrows]Rotate".fg(Color::Gray).into(),
            "[Enter]Scramble".fg(Color::Gray).into(),
            "[Tab]Stats".fg(Color::Gray).into(),
            "[Esc|q]Quit".fg(Color::Gray).into(),
        ])
        .separator("  ")
    }
}

//===========================================================================//
//                        Key handling & game loop                           //
//===========================================================================//
impl App {
    fn game_loop(&mut self) -> Result<(), Error> {
        self.state = State::Playing;
        self.time = Duration::from_secs(0);
        self.moves_cnt = 1;
        self.render()?;

        if self.board.solved() {
            return self.save_stat();
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

        self.save_stat()
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
    fn save_stat(&mut self) -> Result<(), Error> {
        if self.state == State::Playing {
            self.stats.add(Stat::new(
                self.time,
                self.moves_cnt,
                self.moves.clone(),
                self.board.selected,
            ));
            self.stats.save(&self.board.size)?;
            self.state = State::Idle;

            if self.stat_state.borrow().selected > 0 {
                self.stat_state.borrow_mut().selected += 1;
            } else {
                self.load_stat_board()?;
            }
        }
        Ok(())
    }
}
