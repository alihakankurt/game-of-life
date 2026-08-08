mod game;
mod grid;

use clap::Parser;
use crossterm::{
    cursor::{Hide, Show},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
pub use game::Game;
pub use grid::Grid;
use std::{
    io::{self, IsTerminal},
    panic,
    process::exit,
};

#[derive(clap::Parser)]
#[command(version, about, long_about = None)]
#[command(disable_help_flag = true)]
struct Args {
    /// Print help message.
    #[arg(long, action = clap::ArgAction::Help)]
    help: Option<bool>,

    /// The width of the grid.
    #[arg(short = 'w', long, value_parser = clap::value_parser!(u32).range(3..))]
    width: u32,

    /// The height of the grid.
    #[arg(short = 'h', long, value_parser = clap::value_parser!(u32).range(3..))]
    height: u32,

    /// The target fps to run.
    #[arg(long, value_parser = clap::value_parser!(u32).range(1..), default_value_t = 20)]
    fps: u32,
}

fn initialize_panic_handler() {
    let original_hook = panic::take_hook();

    panic::set_hook(Box::new(move |panic_info| {
        let _ = disable_raw_mode();
        let _ = execute!(io::stdout(), LeaveAlternateScreen, Show);

        original_hook(panic_info);
    }));
}

fn main() -> io::Result<()> {
    let mut stdout = io::stdout();

    if !stdout.is_terminal() {
        println!("program is not running on terminal, exiting...");
        exit(1);
    }

    let args = Args::parse();
    let mut game = Game::new(args.width, args.height, args.fps);

    // TODO(@alihakankurt): Read initial state from a file, and construct state to pass.
    let state = vec![1 << 62, 1 << 61, 1 << 63 | 1 << 62 | 1 << 61, 0, 0, 0, 0, 0, 0, 0];
    game.set_state(state);

    execute!(stdout, EnterAlternateScreen, Hide)?;
    enable_raw_mode()?;

    initialize_panic_handler();

    game.run(&mut stdout)?;

    let _ = disable_raw_mode();
    let _ = execute!(stdout, LeaveAlternateScreen, Show);

    return Ok(());
}
