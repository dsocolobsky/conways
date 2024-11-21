use macroquad::rand;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum CellState {
    Dead,
    Alive,
}

pub struct Conways {
    pub(crate) grid: Vec<Vec<CellState>>,
}

impl Conways {
    pub(crate) fn new(width: usize, height: usize, alive: Vec<(usize, usize)>) -> Self {
        let mut conways = Self {
            grid: vec![vec![CellState::Dead; width]; height],
        };
        for (i, j) in alive {
            conways.grid[i][j] = CellState::Alive;
        }
        conways
    }

    fn set_cells_to(&mut self, width: usize, height: usize, alive: Vec<(usize, usize)>) {
        let mut grid = vec![vec![CellState::Dead; width]; height];
        for (i, j) in alive {
            grid[i][j] = CellState::Alive;
        }
        self.grid = grid;
    }

    pub fn set_to_random_grid(&mut self, width: usize, height: usize) {
        let lower_lim = width * height / 30;
        let upper_lim = width * height / 2;
        let num_alive = rand::gen_range(lower_lim, upper_lim);
        let positions =
            (0..num_alive) // Generate #num_alive random positions in grid
                .map(|_| (rand::gen_range(0, height), rand::gen_range(0, width)))
                .collect();
        self.set_cells_to(width, height, positions);
    }

    fn valid_coordinate(&self, row: usize, col: usize) -> bool {
        let (width, height) = self.dimensions();
        row < height && col < width
    }

    pub fn get_cell_state(&self, x: usize, y: usize) -> CellState {
        self.grid
            .get(x)
            .and_then(|row| row.get(y))
            .copied()
            .unwrap_or(CellState::Dead)
    }

    pub fn cell_is_alive(&self, x: usize, y: usize) -> bool {
        self.get_cell_state(x, y) == CellState::Alive
    }

    fn neighbour_positions(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let offsets: [(isize, isize); 8] = [
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
            (-1, -1),
            (-1, 0),
            (-1, 1),
        ];
        offsets
            .iter()
            .filter_map(|(ox, oy)| {
                let nx = (x as isize + *ox) as usize;
                let ny = (y as isize + *oy) as usize;
                if self.valid_coordinate(nx, ny) {
                    Some((nx, ny))
                } else {
                    None
                }
            })
            .collect()
    }

    fn alive_neighbours(&self, x: usize, y: usize) -> usize {
        let n = self
            .neighbour_positions(x, y)
            .into_iter()
            .filter(|(nx, ny)| self.cell_is_alive(*nx, *ny));

        n.count()
    }

    fn next_state_for_cell(&self, x: usize, y: usize) -> CellState {
        let currently_alive = self.cell_is_alive(x, y);
        let neighbours = self.alive_neighbours(x, y);
        if currently_alive {
            if (2..=3).contains(&neighbours) {
                CellState::Alive
            } else {
                CellState::Dead
            }
        } else if neighbours == 3 {
            CellState::Alive
        } else {
            CellState::Dead
        }
    }

    pub fn advance_state(&mut self) {
        let (width, height) = self.dimensions();
        let mut new_grid = vec![vec![CellState::Dead; width]; height];

        for (i, row) in self.grid.iter().enumerate() {
            for (j, _) in row.iter().enumerate() {
                new_grid[i][j] = self.next_state_for_cell(i, j);
            }
        }

        self.grid = new_grid;
    }

    pub fn dimensions(&self) -> (usize, usize) {
        (self.grid[0].len(), self.grid.len())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grid_dimensions() {
        let mut game = Conways::new(32, 16, vec![]);
        assert_eq!((32, 16), game.dimensions());

        game.advance_state();
        assert_eq!((32, 16), game.dimensions());

        game.set_to_random_grid(42, 42);
        assert_eq!((42, 42), game.dimensions());
    }

    #[test]
    fn grid_with_one_cell() {
        let game = Conways::new(32, 32, vec![(5, 5)]);
        for (i, &ref row) in game.grid.iter().enumerate() {
            for (j, &cell) in row.iter().enumerate() {
                if i == 5 && j == 5 {
                    assert_eq!(cell, CellState::Alive)
                } else {
                    assert_eq!(cell, CellState::Dead)
                }
            }
        }
    }

    #[test]
    fn cell_at_corner() {
        let game = Conways::new(32, 32, vec![(0, 0), (0, 1), (1, 0)]);
        assert_eq!(game.alive_neighbours(0, 0), 2);
    }

    #[test]
    fn cell_dies_of_underpopulation() {
        let mut game = Conways::new(32, 32, vec![(5, 5)]);
        assert!(game.cell_is_alive(5, 5));
        game.advance_state();
        assert!(!game.cell_is_alive(5, 5));
    }

    #[test]
    fn cell_dies_of_overpopulation() {
        let mut game = Conways::new(32, 32, vec![(5, 5), (5, 6), (5, 4), (4, 5), (6, 6)]);
        assert!(game.cell_is_alive(5, 5));
        game.advance_state();
        assert!(!game.cell_is_alive(5, 5));
    }

    #[test]
    fn cell_survives() {
        let mut game = Conways::new(32, 32, vec![(5, 5), (5, 6), (5, 4)]);
        assert!(game.cell_is_alive(5, 5));
        game.advance_state();
        assert!(game.cell_is_alive(5, 5));
    }

    #[test]
    fn blinker_pattern() {
        let mut game = Conways::new(32, 32, vec![(5, 5), (5, 6), (5, 7)]);
        assert!(game.cell_is_alive(5, 5));
        assert!(game.cell_is_alive(5, 6));
        assert!(game.cell_is_alive(5, 7));

        game.advance_state();
        assert!(game.cell_is_alive(4, 6));
        assert!(game.cell_is_alive(5, 6));
        assert!(game.cell_is_alive(6, 6));

        assert!(!game.cell_is_alive(5, 5));
        assert!(!game.cell_is_alive(5, 7));

        game.advance_state();
        assert!(game.cell_is_alive(5, 5));
        assert!(game.cell_is_alive(5, 6));
        assert!(game.cell_is_alive(5, 7));

        assert!(!game.cell_is_alive(4, 6));
        assert!(!game.cell_is_alive(6, 6));
    }
}
