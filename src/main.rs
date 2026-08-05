mod game;
mod grid;

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

    execute!(stdout, EnterAlternateScreen, Hide)?;
    enable_raw_mode()?;

    initialize_panic_handler();

    // TODO(@alihakankurt): Update target FPS, width & height etc. as command-line options.
    const TARGET_FPS: u32 = 20;
    let width = 10;
    let height = 10;
    let mut game = Game::new(width, height, TARGET_FPS);

    // TODO(@alihakankurt): Read initial state from a file, and construct state to pass.
    let state = vec![1 << 62, 1 << 61, 1 << 63 | 1 << 62 | 1 << 61, 0, 0, 0, 0, 0, 0, 0];
    game.set_state(state);

    game.run(&mut stdout)?;

    let _ = disable_raw_mode();
    let _ = execute!(stdout, LeaveAlternateScreen, Show);

    return Ok(());
}
