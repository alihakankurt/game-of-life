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

    // TODO(@alihakankurt): Create a way to set initial game state, maybe read from file or through command-line.
    game.grid.set(1, 0);
    game.grid.set(2, 1);
    game.grid.set(2, 2);
    game.grid.set(1, 2);
    game.grid.set(0, 2);
    game.grid.update();

    game.run(&mut stdout)?;

    disable_raw_mode()?;
    execute!(stdout, LeaveAlternateScreen)?;

    return Ok(());
}
