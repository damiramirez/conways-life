const COLUMNS: usize = 32;
const ROWS: usize = 32;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum CellState {
    Alive,
    Dead,
}

type Position = (usize, usize);

pub struct Conways {
    pub grid: Vec<Vec<CellState>>,
}

impl Conways {
    pub fn from(alive_cells: Vec<Position>) -> Self {
        let mut grid = vec![vec![CellState::Dead; COLUMNS]; ROWS];

        for (x, y) in alive_cells {
            if x < ROWS && y < COLUMNS {
                grid[x][y] = CellState::Alive;
            }
        }

        Self { grid }
    }

    pub fn update_cells(&mut self) {
        let mut new_grid = self.grid.clone();

        for (x, _) in self.grid.iter().enumerate() {
            for (y, _) in self.grid.iter().enumerate() {
                let neighbors = self.count_neighbors((x, y));
                new_grid[x][y] = match (self.grid[x][y], neighbors) {
                    (CellState::Alive, 2 | 3) => CellState::Alive,
                    (CellState::Alive, _) => CellState::Dead,
                    (CellState::Dead, 3) => CellState::Alive,
                    (CellState::Dead, _) => CellState::Dead,
                };
            }
        }

        self.grid = new_grid;
    }

    pub fn count_neighbors(&self, position: Position) -> usize {
        let mut count = 0;

        // Position of the eight neighbors
        let directions = [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ];

        for (x, y) in directions.iter() {
            let new_x = position.0 as isize + x;
            let new_y = position.1 as isize + y;

            // Avoid leaving the grid and check if the cell is alive
            if new_x >= 0
                && new_x < ROWS as isize
                && new_y >= 0
                && new_y < COLUMNS as isize
                && self.grid[new_x as usize][new_y as usize] == CellState::Alive
            {
                count += 1;
            }
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_grid() {
        let conways = Conways::from(vec![]);
        for row in conways.grid.iter() {
            for &cell in row.iter() {
                assert_eq!(cell, CellState::Dead);
            }
        }
    }

    #[test]
    fn test_from_with_alive_cells() {
        let alive_cells = vec![(1, 1), (2, 2), (3, 3)];
        let conways = Conways::from(alive_cells);
        assert_eq!(conways.grid[1][1], CellState::Alive);
        assert_eq!(conways.grid[2][2], CellState::Alive);
        assert_eq!(conways.grid[3][3], CellState::Alive);
        assert_eq!(conways.grid[2][1], CellState::Dead);
    }

    #[test]
    fn test_count_neighbors() {
        let alive_cells = vec![(1, 1), (1, 2), (1, 3)];
        let conways = Conways::from(alive_cells);
        assert_eq!(conways.count_neighbors((0, 0)), 1);
        assert_eq!(conways.count_neighbors((1, 1)), 1);
        assert_eq!(conways.count_neighbors((1, 2)), 2);
        assert_eq!(conways.count_neighbors((2, 2)), 3);
    }

    #[test]
    fn test_blinker_patron() {
        let alive_cells = vec![(0, 1), (1, 1), (2, 1)];
        let mut conways = Conways::from(alive_cells);
        conways.update_cells();
        assert_eq!(CellState::Alive, conways.grid[1][2]);
        assert_eq!(CellState::Alive, conways.grid[1][0]);
        assert_eq!(CellState::Alive, conways.grid[1][1]);
        assert_eq!(CellState::Dead, conways.grid[0][1]);
    }
}
