mod game;
mod grid;

use std::{
    io::{self, stdout},
    time::{Duration, Instant},
};

use crossterm::{
    cursor::{MoveToColumn, MoveToRow},
    event::{self, Event, KeyCode},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
pub use game::Game;
pub use grid::Grid;

fn main() -> io::Result<()> {
    let mut stdout = stdout();

    // TODO(@alihakankurt): Update target FPS, width & height etc. as command-line options.
    const TARGET_FPS: f64 = 20.0;
    let target_frame_time = Duration::from_secs_f64(1.0 / TARGET_FPS);

    let width = 10;
    let height = 10;
    let mut game = Game::new(width, height);

    // TODO(@alihakankurt): Create a way to set initial game state, maybe read from file or through command-line.
    game.grid.set(1, 0);
    game.grid.set(2, 1);
    game.grid.set(2, 2);
    game.grid.set(1, 2);
    game.grid.set(0, 2);
    game.grid.update();

    execute!(stdout, EnterAlternateScreen)?;
    enable_raw_mode()?;

    execute!(stdout, MoveToRow(0), MoveToColumn(0))?;
    game.render();

    // TODO(@alihakankurt): Move this main loop into a method inside Game struct.
    let mut running = true;
    while running {
        let start_time = Instant::now();

        game.update();

        execute!(stdout, MoveToRow(0), MoveToColumn(0))?;
        game.render();

        while event::poll(Duration::from_millis(0))? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Esc => {
                        running = false;
                        break;
                    }
                    _ => {}
                }
            }
        }

        let frame_time = start_time.elapsed();
        if frame_time < target_frame_time {
            std::thread::sleep(target_frame_time - frame_time);
        }
    }

    disable_raw_mode()?;
    execute!(stdout, LeaveAlternateScreen)?;
    Ok(())
}
