use crate::Grid;

pub struct Game {
    pub grid: Grid,
}

impl Game {
    pub fn new(width: u32, height: u32) -> Self {
        return Game {
            grid: Grid::new(width, height),
        };
    }

    pub fn update(&mut self) {
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

    pub fn render(&self) {
        for y in 0..self.grid.height {
            for x in 0..self.grid.width {
                let ch = match self.grid.get(x, y) {
                    0b1 => "#",
                    _ => ".",
                };
                print!("{}", ch);
            }
            println!();
        }
    }
}
