pub struct Grid {
    pub width: usize,
    pub height: usize,
    data: Vec<u8>,
}

impl Grid {
    pub fn new(width: usize, height: usize) -> Self {
        Grid {
            width,
            height,
            data: vec![0u8; width * height],
        }
    }

    pub fn get(&self, x: usize, y: usize) -> u8 {
        let index = self.calculate_index(x, y);
        return self.data[index] & 0x01;
    }

    pub fn set(&mut self, x: usize, y: usize) {
        let index = self.calculate_index(x, y);
        self.data[index] |= 0x2;
    }

    pub fn reset(&mut self, x: usize, y: usize) {
        let index = self.calculate_index(x, y);
        self.data[index] &= !0x2;
    }

    pub fn update(&mut self) {
        for value in &mut self.data {
            *value >>= 1;
        }
    }

    pub fn count(&self, x: usize, y: usize) -> u8 {
        let x_left = if x == 0 { self.width - 1 } else { x - 1 };
        let x_right = if x == self.width - 1 { 0 } else { x + 1 };

        let y_top = if y == 0 { self.height - 1 } else { y - 1 };
        let y_bottom = if y == self.height - 1 { 0 } else { y + 1 };

        let mut neighbors = 0;
        neighbors += self.get(x_left, y_top) + self.get(x, y_top) + self.get(x_right, y_top);
        neighbors += self.get(x_left, y) + self.get(x_right, y);
        neighbors += self.get(x_left, y_bottom) + self.get(x, y_bottom) + self.get(x_right, y_bottom);
        return neighbors;
    }

    fn calculate_index(&self, x: usize, y: usize) -> usize {
        return y * self.width + x;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grid_set() {
        let mut grid = Grid::new(4, 4);

        grid.set(1, 2);
        assert_eq!(grid.get(1, 2), 0);

        grid.update();
        assert_eq!(grid.get(1, 2), 1);
    }

    #[test]
    fn test_grid_reset() {
        let mut grid = Grid::new(4, 4);
        grid.set(2, 1);
        grid.update();

        grid.reset(2, 1);
        assert_eq!(grid.get(2, 1), 1);

        grid.update();
        assert_eq!(grid.get(2, 1), 0);
    }

    #[test]
    fn test_grid_count_neighbors() {
        let mut grid = Grid::new(7, 7);

        grid.set(1, 1);
        grid.set(1, 0);
        grid.set(0, 6);
        grid.set(6, 6);
        grid.update();

        assert_eq!(grid.count(0, 0), 4);
    }
}
