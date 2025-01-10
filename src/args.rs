use termint::{
    enums::Color,
    help,
    widgets::{Grad, StrSpanExtension},
};

use crate::{error::Error, size::Size};

#[derive(Debug, Eq, PartialEq, Clone, Default)]
pub enum Action {
    #[default]
    Play,
    Config,
    Help,
}

/// Parses given arguments and checks for arguments conditions
#[derive(Debug, Default)]
pub struct Args {
    pub size: Option<Size>,
    pub action: Action,
}

impl Args {
    /// Parses arguments
    pub fn parse(args: std::env::Args) -> Result<Args, Error> {
        let mut parsed = Self::default();

        let mut args_iter = args.into_iter();
        args_iter.next();
        while let Some(arg) = args_iter.next() {
            match arg.as_str() {
                "-s" | "--size" => parsed.parse_size(&mut args_iter)?,
                "-h" | "--help" => parsed.action = Action::Help,
                "config" => parsed.action = Action::Config,
                arg => Err(format!("unexpected argument: '{arg}'"))?,
            }
        }

        Ok(parsed)
    }

    /// Displays help
    pub fn help() {
        println!(
            "Welcome to help for {} by {}\n",
            "loopover".fg(Color::Green),
            Grad::new("Martan03", (0, 220, 255), (175, 80, 255))
        );
        help!(
            "Usage":
            "loopover" => "Opens 3x3 game\n"
            "loopover" ["options"] => "Behaves according to options\n"
            "Options":
            "-s  --size" ["width"] ["height"] => "Sets size of the game\n"
            "-h  --help" => "Prints this help"
        );
    }

    /// Parses size from the given arguments
    fn parse_size<T>(&mut self, args: &mut T) -> Result<(), Error>
    where
        T: Iterator<Item = String>,
    {
        let size = Size::new(Args::get_num(args)?, Args::get_num(args)?);
        if size.width < 2 || size.height < 2 {
            return Err(Error::Msg("minimum supported size is 2".into()));
        }
        self.size = Some(size);
        Ok(())
    }

    /// Gets number (usize) from args
    fn get_num<T>(args: &mut T) -> Result<usize, Error>
    where
        T: Iterator<Item = String>,
    {
        let Some(val) = args.next() else {
            return Err(Error::Msg("missing argument parameter".into()));
        };

        val.parse::<usize>()
            .map_err(|_| Error::Msg(format!("number expected, got '{val}'")))
    }
}
