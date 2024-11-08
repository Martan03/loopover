use std::{
    env,
    fs::create_dir_all,
    process::{Command, ExitCode},
};

use app::App;
use args::{Action, Args};
use config::Config;
use error::Error;
use termint::{enums::Color, widgets::StrSpanExtension};

mod app;
mod args;
mod board;
mod config;
mod error;
mod size;
mod stats;

fn main() -> ExitCode {
    match run() {
        Ok(_) => ExitCode::SUCCESS,
        Err(e) => {
            println!("{} {e}", "Error:".fg(Color::Red));
            ExitCode::FAILURE
        }
    }
}

fn run() -> Result<(), Error> {
    let args = Args::parse(std::env::args())?;
    match args.action {
        Action::Play => run_play(args),
        Action::Config => run_config(),
        Action::Help => Ok(Args::help()),
    }
}

fn run_play(args: Args) -> Result<(), Error> {
    let config = Config::load();
    let size = args.size.unwrap_or(config.default_size);

    let mut app = App::new(size.into());
    app.run()
}

fn run_config() -> Result<(), Error> {
    let editor = env::var("EDITOR").unwrap_or("vi".to_string());
    create_dir_all(Config::get_dir())?;
    let file = Config::get_path();
    if !file.exists() {
        Config::default().save()?;
    }

    Command::new(editor).arg(file).spawn()?.wait()?;
    Ok(())
}
