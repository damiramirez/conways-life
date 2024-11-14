const COLUMNS: usize = 32;
const ROWS: usize = 32;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum CellState {
    Alive,
    Dead,
}

#[derive(Clone, Copy)]
pub struct Position(pub usize, pub usize);

pub struct Conways {
    pub grid: Vec<Vec<CellState>>,
}

impl Conways {

    pub fn from(alive_cells: Vec<Position>) -> Self {
        let mut grid = vec![vec![CellState::Dead; COLUMNS]; ROWS];
        
        for Position(x, y) in alive_cells {
            if x < ROWS && y < COLUMNS {
                grid[x][y] = CellState::Alive;
            }
        }
        
        Self { grid }
    }

    pub fn update_cells(&mut self) {
        let mut new_grid = self.grid.clone();

        // Iterate over the grid, updating with the new state
        for x in 0..self.grid.len() {
            for y in 0..self.grid.len() {
                let neighbors = self.count_neighbors(Position(x, y));
                let cell = self.grid[x][y];
                match cell {
                    CellState::Alive => {
                        if neighbors > 3 || neighbors < 2 {
                            new_grid[x][y] = CellState::Dead;
                        }
                    }
                    CellState::Dead => {
                        if neighbors == 3 {
                            new_grid[x][y] = CellState::Alive;
                        }
                    }
                }
            }
        }
        
        self.grid = new_grid;
    }

    pub fn count_neighbors(&self, position: Position) -> usize {
        let mut count = 0;

        // Position of the eight neighbors
        let directions = [
            (-1, -1), (-1, 0), (-1, 1),
            (0, -1),           (0, 1),
            (1, -1),  (1, 0),   (1, 1),
        ];

        for (x, y) in directions.iter() {
            let new_x = position.0 as isize + x;
            let new_y = position.1 as isize + y;

            // Avoid leaving the grid
            if new_x >= 0 && new_x < ROWS as isize && new_y >= 0 && new_y < COLUMNS as isize {
                if let CellState::Alive = self.grid[new_x as usize][new_y as usize] {
                    count += 1;
                }
            }
        }
        count
    }
}