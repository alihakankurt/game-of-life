use clap::{ArgAction::Help, Parser};

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(disable_help_flag = true)]
pub struct Args {
    /// Print help message.
    #[arg(long, action = Help)]
    pub help: Option<bool>,

    /// The width of the grid.
    #[arg(short = 'w', long, value_parser = Self::parse_dimension)]
    pub width: usize,

    /// The height of the grid.
    #[arg(short = 'h', long, value_parser = Self::parse_dimension)]
    pub height: usize,

    /// The target fps to run.
    #[arg(long, value_parser = Self::parse_fps)]
    pub fps: f64,

    /// The file path to initial state.
    #[arg(short = 'f', long)]
    pub file: String,
}

impl Args {
    fn parse_dimension(s: &str) -> Result<usize, String> {
        let n = s.parse::<usize>().map_err(|_| "must be a positive value")?;
        if n < 3usize {
            return Err("must be at least 3".into());
        }

        return Ok(n);
    }

    fn parse_fps(s: &str) -> Result<f64, String> {
        let n = s.parse::<f64>().map_err(|_| "must be a positive value")?;
        if n < 1.0f64 {
            return Err("must be at least 1".into());
        }

        return Ok(n);
    }
}
