mod game;
mod grid;

use crossterm::{
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
pub use game::Game;
pub use grid::Grid;
use std::io;

fn main() -> io::Result<()> {
    let mut stdout = io::stdout();

    execute!(stdout, EnterAlternateScreen)?;
    enable_raw_mode()?;

    // TODO(@alihakankurt): Update target FPS, width & height etc. as command-line options.
    const TARGET_FPS: u32 = 20;
    let width = 10;
    let height = 10;
    let mut game = Game::new(width, height, TARGET_FPS);

    // TODO(@alihakankurt): Read initial state from a file, and construct state to pass.
    let state = vec![1 << 62, 1 << 61, 1 << 63 | 1 << 62 | 1 << 61, 0, 0, 0, 0, 0, 0, 0];
    game.set_state(state);

    game.run(&mut stdout)?;

    disable_raw_mode()?;
    execute!(stdout, LeaveAlternateScreen)?;

    return Ok(());
}
