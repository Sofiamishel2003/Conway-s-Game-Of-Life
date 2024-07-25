pub struct GameOfLife {
    width: usize,
    height: usize,
    cells: Vec<Vec<bool>>,
}

impl GameOfLife {
    pub fn new(width: usize, height: usize) -> GameOfLife {
        GameOfLife {
            width,
            height,
            cells: vec![vec![false; width]; height],
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn is_alive(&self, x: usize, y: usize) -> bool {
        self.cells[y][x]
    }

    pub fn set_alive(&mut self, x: usize, y: usize) {
        self.cells[y][x] = true;
    }

    pub fn set_dead(&mut self, x: usize, y: usize) {
        self.cells[y][x] = false;
    }

    pub fn update(&mut self) {
        let mut new_cells = self.cells.clone();

        for y in 0..self.height {
            for x in 0..self.width {
                let alive_neighbors = self.alive_neighbors(x, y);

                if self.cells[y][x] {
                    if alive_neighbors < 2 || alive_neighbors > 3 {
                        new_cells[y][x] = false;
                    }
                } else {
                    if alive_neighbors == 3 {
                        new_cells[y][x] = true;
                    }
                }
            }
        }

        self.cells = new_cells;
    }

    fn alive_neighbors(&self, x: usize, y: usize) -> usize {
        let mut count = 0;
        let dirs = [
            (-1, -1), (0, -1), (1, -1),
            (-1, 0),         (1, 0),
            (-1, 1), (0, 1), (1, 1),
        ];

        for (dx, dy) in dirs.iter() {
            let nx = (x as isize + dx).rem_euclid(self.width as isize) as usize;
            let ny = (y as isize + dy).rem_euclid(self.height as isize) as usize;

            if self.cells[ny][nx] {
                count += 1;
            }
        }

        count
    }

    pub fn set_block(&mut self, x: usize, y: usize) {
        let pattern = [(0, 0), (1, 0), (0, 1), (1, 1)];
        for (dx, dy) in &pattern {
            self.set_alive(x + dx, y + dy);
        }
    }

    pub fn set_beehive(&mut self, x: usize, y: usize) {
        let pattern = [(1, 0), (2, 0), (0, 1), (3, 1), (1, 2), (2, 2)];
        for (dx, dy) in &pattern {
            self.set_alive(x + dx, y + dy);
        }
    }

    pub fn set_loaf(&mut self, x: usize, y: usize) {
        let pattern = [(1, 0), (2, 0), (0, 1), (3, 1), (1, 2), (3, 2), (2, 3)];
        for (dx, dy) in &pattern {
            self.set_alive(x + dx, y + dy);
        }
    }

    pub fn set_boat(&mut self, x: usize, y: usize) {
        let pattern = [(0, 0), (1, 0), (0, 1), (2, 1), (1, 2)];
        for (dx, dy) in &pattern {
            self.set_alive(x + dx, y + dy);
        }
    }

    pub fn set_tub(&mut self, x: usize, y: usize) {
        let pattern = [(1, 0), (0, 1), (2, 1), (1, 2)];
        for (dx, dy) in &pattern {
            self.set_alive(x + dx, y + dy);
        }
    }

    pub fn set_blinker(&mut self, x: usize, y: usize) {
        let pattern = [(0, 1), (1, 1), (2, 1)];
        for (dx, dy) in &pattern {
            self.set_alive(x + dx, y + dy);
        }
    }

    pub fn set_toad(&mut self, x: usize, y: usize) {
        let pattern = [(1, 0), (2, 0), (3, 0), (0, 1), (1, 1), (2, 1)];
        for (dx, dy) in &pattern {
            self.set_alive(x + dx, y + dy);
        }
    }

    pub fn set_beacon(&mut self, x: usize, y: usize) {
        let pattern = [(0, 0), (1, 0), (0, 1), (3, 2), (2, 3), (3, 3)];
        for (dx, dy) in &pattern {
            self.set_alive(x + dx, y + dy);
        }
    }

    pub fn set_pulsar(&mut self, x: usize, y: usize) {
        let pattern = [
            (2, 0), (3, 0), (4, 0), (8, 0), (9, 0), (10, 0),
            (0, 2), (5, 2), (7, 2), (12, 2),
            (0, 3), (5, 3), (7, 3), (12, 3),
            (0, 4), (5, 4), (7, 4), (12, 4),
            (2, 5), (3, 5), (4, 5), (8, 5), (9, 5), (10, 5),
            (2, 7), (3, 7), (4, 7), (8, 7), (9, 7), (10, 7),
            (0, 8), (5, 8), (7, 8), (12, 8),
            (0, 9), (5, 9), (7, 9), (12, 9),
            (0, 10), (5, 10), (7, 10), (12, 10),
            (2, 12), (3, 12), (4, 12), (8, 12), (9, 12), (10, 12),
        ];
        for (dx, dy) in &pattern {
            self.set_alive(x + dx, y + dy);
        }
    }

    pub fn set_pentadecathlon(&mut self, x: usize, y: usize) {
        let pattern = [
            (1, 0), (2, 0),
            (0, 1), (3, 1),
            (1, 2), (2, 2),
            (1, 3), (2, 3),
            (1, 4), (2, 4),
            (1, 5), (2, 5),
            (0, 6), (3, 6),
            (1, 7), (2, 7),
        ];
        for (dx, dy) in &pattern {
            self.set_alive(x + dx, y + dy);
        }
    }

    pub fn set_glider(&mut self, x: usize, y: usize) {
        let pattern = [(1, 0), (2, 1), (0, 2), (1, 2), (2, 2)];
        for (dx, dy) in &pattern {
            self.set_alive(x + dx, y + dy);
        }
    }

    pub fn set_lwss(&mut self, x: usize, y: usize) {
        let pattern = [(1, 0), (4, 0), (0, 1), (0, 2), (4, 2), (0, 3), (1, 3), (2, 3), (3, 3)];
        for (dx, dy) in &pattern {
            self.set_alive(x + dx, y + dy);
        }
    }

    pub fn set_mwss(&mut self, x: usize, y: usize) {
        let pattern = [
            (1, 0), (2, 0), (3, 0), (4, 0),
            (0, 1), (0, 2), (4, 2), (5, 2),
            (0, 3), (1, 3), (2, 3), (3, 3), (5, 3),
        ];
        for (dx, dy) in &pattern {
            self.set_alive(x + dx, y + dy);
        }
    }

    pub fn set_hwss(&mut self, x: usize, y: usize) {
        let pattern = [
            (1, 0), (2, 0), (3, 0), (4, 0), (5, 0),
            (0, 1), (0, 2), (5, 2), (6, 2),
            (0, 3), (1, 3), (2, 3), (3, 3), (4, 3), (6, 3),
        ];
        for (dx, dy) in &pattern {
            self.set_alive(x + dx, y + dy);
        }
    }
}
