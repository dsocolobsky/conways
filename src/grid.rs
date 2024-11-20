use macroquad::rand;

pub const GRID_WIDTH: usize = 32;
pub const GRID_HEIGHT: usize = 32;
pub const CELL_SIZE: f32 = 30.0;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum CellState {
    Dead,
    Alive,
}

pub type Grid = Vec<Vec<CellState>>;

fn valid_coordinate(grid: &Grid, row: usize, col: usize) -> bool {
    let (width, height) = dimensions(grid);
    row < height && col < width
}

pub fn get_cell_state(grid: &Grid, x: usize, y: usize) -> CellState {
    if valid_coordinate(grid, x, y) {
        grid[x][y]
    } else {
        CellState::Dead
    }
}

pub fn cell_is_alive(grid: &Grid, x: usize, y: usize) -> bool {
    get_cell_state(grid, x, y) == CellState::Alive
}

fn neighbour_positions(grid: &Grid, x: usize, y: usize) -> Vec<(usize, usize)> {
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
            if valid_coordinate(grid, nx, ny) {
                Some((nx, ny))
            } else {
                None
            }
        })
        .collect()
}

fn alive_neighbours(grid: &Grid, x: usize, y: usize) -> usize {
    let n = neighbour_positions(grid, x, y)
        .into_iter()
        .filter(|(nx, ny)| cell_is_alive(grid, *nx, *ny));

    n.count()
}

fn next_state_for_cell(grid: &Grid, x: usize, y: usize) -> CellState {
    let currently_alive = cell_is_alive(grid, x, y);
    let neighbours = alive_neighbours(grid, x, y);
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

pub fn next_state_for_grid(grid: &Grid) -> Grid {
    let (width, height) = dimensions(grid);
    let mut new_grid: Grid = vec![vec![CellState::Dead; width]; height];

    for (i, row) in grid.iter().enumerate() {
        for (j, _) in row.iter().enumerate() {
            new_grid[i][j] = next_state_for_cell(grid, i, j);
        }
    }

    new_grid
}

pub fn create_grid(width: usize, height: usize, alive: Vec<(usize, usize)>) -> Grid {
    let mut new_grid: Grid = vec![vec![CellState::Dead; width]; height];
    for (i, j) in alive {
        new_grid[i][j] = CellState::Alive;
    }
    new_grid
}

pub fn create_random_grid(width: usize, height: usize) -> Grid {
    let lower_lim = width * height / 30;
    let upper_lim = width * height / 2;
    let num_alive = rand::gen_range(lower_lim, upper_lim);
    let positions = (0..num_alive) // Generate #num_alive random positions in grid
        .map(|_| (rand::gen_range(0, height), rand::gen_range(0, width)))
        .collect();
    create_grid(width, height, positions)
}

pub fn dimensions(grid: &Grid) -> (usize, usize) {
    (grid[0].len(), grid.len())
}

#[cfg(test)]
mod tests {
    use crate::grid::{
        alive_neighbours, cell_is_alive, create_grid, next_state_for_grid, CellState,
    };

    #[test]
    fn test_grid_dimensions() {
        let grid = create_grid(32, 16, vec![]);
        assert_eq!(grid.len(), 16);
        assert_eq!(grid[0].len(), 32);

        let grid = next_state_for_grid(&grid);
        assert_eq!(grid.len(), 16);
        assert_eq!(grid[0].len(), 32);
    }

    #[test]
    fn grid_with_one_cell() {
        let grid = create_grid(32, 32, vec![(5, 5)]);
        for (i, &ref row) in grid.iter().enumerate() {
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
        let grid = create_grid(32, 32, vec![(0, 0), (0, 1), (1, 0)]);
        assert_eq!(alive_neighbours(&grid, 0, 0), 2);
    }

    #[test]
    fn cell_dies_of_underpopulation() {
        let grid = create_grid(32, 32, vec![(5, 5)]);
        assert!(cell_is_alive(&grid, 5, 5));
        let grid = next_state_for_grid(&grid);
        assert!(!cell_is_alive(&grid, 5, 5));
    }

    #[test]
    fn cell_dies_of_overpopulation() {
        let grid = create_grid(32, 32, vec![(5, 5), (5, 6), (5, 4), (4, 5), (6, 6)]);
        assert!(cell_is_alive(&grid, 5, 5));
        let grid = next_state_for_grid(&grid);
        assert!(!cell_is_alive(&grid, 5, 5));
    }

    #[test]
    fn cell_survives() {
        let grid = create_grid(32, 32, vec![(5, 5), (5, 6), (5, 4)]);
        assert!(cell_is_alive(&grid, 5, 5));
        let grid = next_state_for_grid(&grid);
        assert!(cell_is_alive(&grid, 5, 5));
    }

    #[test]
    fn blinker_pattern() {
        let grid = create_grid(32, 32, vec![(5, 5), (5, 6), (5, 7)]);
        assert!(cell_is_alive(&grid, 5, 5));
        assert!(cell_is_alive(&grid, 5, 6));
        assert!(cell_is_alive(&grid, 5, 7));

        let grid = next_state_for_grid(&grid);
        assert!(cell_is_alive(&grid, 4, 6));
        assert!(cell_is_alive(&grid, 5, 6));
        assert!(cell_is_alive(&grid, 6, 6));

        assert!(!cell_is_alive(&grid, 5, 5));
        assert!(!cell_is_alive(&grid, 5, 7));

        let grid = next_state_for_grid(&grid);
        assert!(cell_is_alive(&grid, 5, 5));
        assert!(cell_is_alive(&grid, 5, 6));
        assert!(cell_is_alive(&grid, 5, 7));

        assert!(!cell_is_alive(&grid, 4, 6));
        assert!(!cell_is_alive(&grid, 6, 6));
    }
}
