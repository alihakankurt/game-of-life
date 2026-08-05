use std::{
    io,
    time::{Duration, Instant},
};

use crossterm::{
    cursor::{MoveToColumn, MoveToRow},
    event, execute,
};

use crate::Grid;

pub struct Game {
    pub grid: Grid,
    pub fps: u32,
}

impl Game {
    pub fn new(width: u32, height: u32, fps: u32) -> Self {
        let grid = Grid::new(width, height);
        return Game { grid, fps };
    }

    pub fn run(&mut self, stdout: &mut io::Stdout) -> io::Result<()> {
        let mut running = true;
        let frame_time = Duration::from_secs_f64(1.0 / self.fps as f64);

        execute!(stdout, MoveToRow(0), MoveToColumn(0))?;
        self.render();

        while running {
            let start_time = Instant::now();

            self.update();

            execute!(stdout, MoveToRow(0), MoveToColumn(0))?;
            self.render();

            while event::poll(Duration::from_millis(0))? {
                if let event::Event::Key(key_event) = event::read()? {
                    match key_event.code {
                        event::KeyCode::Esc => {
                            running = false;
                            break;
                        }
                        _ => {}
                    }
                }
            }

            let current_frame_time = start_time.elapsed();
            if current_frame_time < frame_time {
                std::thread::sleep(frame_time - current_frame_time);
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

    fn render(&self) {
        for y in 0..self.grid.height {
            for x in 0..self.grid.width {
                let ch = match self.grid.get(x, y) {
                    0b1 => "#",
                    _ => ".",
                };
                print!("{}", ch);
            }
            print!("\r\n");
        }
    }
}
