use std::{
    io::{self, Write},
    time::{Duration, Instant},
};

use crossterm::{
    cursor::{MoveToColumn, MoveToRow},
    event, execute,
};

use crate::Grid;

pub struct Game {
    grid: Grid,
    pub fps: f64,
}

impl Game {
    pub fn new(grid: Grid, fps: f64) -> Self {
        return Game { grid, fps };
    }

    pub fn run(&mut self, stdout: &mut io::Stdout) -> io::Result<()> {
        let mut running = true;
        let target_frame_duration = Duration::from_secs_f64(1.0f64 / self.fps);

        self.render(stdout)?;
        while running {
            let frame_start_time = Instant::now();

            self.update();
            self.render(stdout)?;

            while event::poll(target_frame_duration.saturating_sub(frame_start_time.elapsed()))? {
                if let event::Event::Key(key_event) = event::read()? {
                    if key_event.code == event::KeyCode::Esc {
                        running = false;
                        break;
                    }
                }
            }
        }

        return Ok(());
    }

    fn update(&mut self) {
        for y in 0..self.grid.height {
            for x in 0..self.grid.width {
                let state = self.grid.get(x, y) << 4 | self.grid.count(x, y);
                match state {
                    0b10010 | 0b10011 | 0b00011 => self.grid.set(x, y),
                    _ => self.grid.reset(x, y),
                }
            }
        }
        self.grid.update();
    }

    fn render(&self, stdout: &mut io::Stdout) -> io::Result<()> {
        execute!(stdout, MoveToRow(0), MoveToColumn(0))?;

        for y in 0..self.grid.height {
            for x in 0..self.grid.width {
                let ch = match self.grid.get(x, y) {
                    0b1 => "#",
                    _ => ".",
                };
                write!(stdout, "{}", ch)?;
            }
            write!(stdout, "\r\n")?;
        }

        return Ok(());
    }
}
