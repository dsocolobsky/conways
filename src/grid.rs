pub const GRID_WIDTH: usize = 32;
pub const GRID_HEIGHT: usize = 32;
pub const CELL_SIZE: f32 = 30.0;

#[derive(Copy, Clone, PartialEq, Default)]
pub enum CellState {
    #[default]
    Dead,
    Alive,
}

pub type Grid = [[CellState; GRID_WIDTH]; GRID_HEIGHT];

fn valid_coordinate(x: usize, y: usize) -> bool {
    x < GRID_WIDTH && y < GRID_HEIGHT
}

pub fn get_cell_state(grid: &Grid, x: usize, y: usize) -> Option<CellState> {
    if valid_coordinate(x, y) {
        Some(grid[x][y])
    } else {
        None
    }
}

fn neighbours(grid: &Grid, x: usize, y: usize) -> Vec<Option<CellState>> {
    let mut vec: Vec<Option<CellState>> = vec![];
    if x > 0 {
        vec.push(get_cell_state(&grid, x - 1, y));
        if y > 0 {
            vec.push(get_cell_state(&grid, x - 1, y - 1));
        }
        if y < 255 {
            vec.push(get_cell_state(&grid, x - 1, y + 1));
        }
    }
    if x < 255 {
        vec.push(get_cell_state(&grid, x + 1, y));
        if y > 0 {
            vec.push(get_cell_state(&grid, x + 1, y - 1));
        }
        if y < 255 {
            vec.push(get_cell_state(&grid, x + 1, y + 1));
        }
    }
    if y > 0 {
        vec.push(get_cell_state(&grid, x, y - 1));
    }
    if y < 255 {
        vec.push(get_cell_state(&grid, x, y + 1));
    }
    vec
}

fn alive_neighbours(grid: &Grid, x: usize, y: usize) -> usize {
    neighbours(grid, x, y)
        .into_iter()
        .flatten()
        .filter(|state| *state == CellState::Alive)
        .count()
}

fn next_state_for_cell(grid: &Grid, x: usize, y: usize) -> CellState {
    let Some(cell_state) = get_cell_state(grid, x, y) else {
        panic!("Invalid coordinate");
    };
    let neighbours = alive_neighbours(grid, x, y);
    if cell_state == CellState::Alive {
        if neighbours < 2 || neighbours > 3 {
            CellState::Dead
        } else {
            CellState::Alive
        }
    } else {
        if neighbours == 3 {
            CellState::Alive
        } else {
            CellState::Dead
        }
    }
}

pub fn next_state_for_grid(grid: &Grid) -> Grid {
    let mut new_grid: Grid = [[CellState::Dead; GRID_WIDTH]; GRID_HEIGHT];

    for (i, row) in grid.iter().enumerate() {
        for (j, _) in row.iter().enumerate() {
            new_grid[i][j] = next_state_for_cell(grid, i, j);
        }
    }

    new_grid
}

pub fn create_initial_grid(alive: Vec<(usize, usize)>) -> Grid {
    let mut new_grid: Grid = [[CellState::Dead; GRID_WIDTH]; GRID_HEIGHT];
    for (i, j) in alive {
        new_grid[i][j] = CellState::Alive;
    }
    new_grid
}
