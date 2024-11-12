use macroquad::prelude::*;

const GRID_WIDTH: usize = 32;
const GRID_HEIGHT: usize = 32;
const CELL_SIZE: f32 = 30.0;

#[derive(Copy, Clone, PartialEq, Default)]
enum CellState {
    #[default]
    Dead,
    Alive,
}

fn window_conf() -> Conf {
    Conf {
        window_title: String::from("Conways"),
        window_width: 1024,
        window_height: 1024,
        fullscreen: false,
        ..Default::default()
    }
}

type Grid = [[CellState; GRID_WIDTH]; GRID_HEIGHT];

fn valid_coordinate(x: usize, y: usize) -> bool {
    x < GRID_WIDTH && y < GRID_HEIGHT
}

fn get_state(grid: &Grid, x: usize, y: usize) -> Option<CellState> {
    if valid_coordinate(x, y) {
        Some(grid[x][y])
    } else {
        None
    }
}

fn set_state(grid: &mut Grid, x: usize, y: usize, state: CellState) {
    if !valid_coordinate(x, y) {
        panic!("Invalid coordinate");
    }
    grid[x][y] = state;
}

fn set_alive(grid: &mut Grid, x: usize, y: usize) {
    set_state(grid, x, y, CellState::Alive);
}

fn set_dead(grid: &mut Grid, x: usize, y: usize) {
    set_state(grid, x, y, CellState::Dead);
}

fn neighbours(grid: &Grid, x: usize, y: usize) -> [Option<CellState>; 8] {
    [
        get_state(&grid, x, y - 1),
        get_state(&grid, x, y + 1),
        get_state(&grid, x - 1, y),
        get_state(&grid, x + 1, y),
        get_state(&grid, x - 1, y - 1),
        get_state(&grid, x - 1, y + 1),
        get_state(&grid, x + 1, y - 1),
        get_state(&grid, x + 1, y + 1),
    ]
}

fn alive_neighbours(grid: &Grid, x: usize, y: usize) -> usize {
    neighbours(grid, x, y)
        .into_iter()
        .flatten()
        .filter(|state| *state == CellState::Alive)
        .count()
}

fn next_state_for_cell(grid: &Grid, x: usize, y: usize) -> CellState {
    let Some(cell_state) = get_state(grid, x, y) else {
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

fn next_state_for_grid(grid: &Grid) -> Grid {
    let mut new_grid: Grid = [[CellState::Dead; GRID_WIDTH]; GRID_HEIGHT];

    for i in 0..GRID_WIDTH {
        for j in 0..GRID_HEIGHT {
            new_grid[i][j] = next_state_for_cell(grid, i, j);
        }
    }

    new_grid
}

fn create_initial_grid(alive: Vec<(usize, usize)>) -> Grid {
    let mut new_grid: Grid = [[CellState::Dead; GRID_WIDTH]; GRID_HEIGHT];
    for (i, j) in alive {
        set_alive(&mut new_grid, i, j);
    }
    new_grid
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut grid: Grid = create_initial_grid(vec![(4, 4), (10, 10)]);

    loop {
        clear_background(DARKGRAY);

        if is_key_released(KeyCode::Space) {
            grid = next_state_for_grid(&grid);
        }

        for i in 0..GRID_WIDTH {
            for j in 0..GRID_HEIGHT {
                let state = get_state(&grid, i, j).unwrap_or_default();
                let color = if state == CellState::Alive {
                    YELLOW
                } else {
                    LIGHTGRAY
                };
                draw_rectangle(
                    (GRID_WIDTH * i) as f32,
                    (GRID_HEIGHT * j) as f32,
                    CELL_SIZE,
                    CELL_SIZE,
                    color,
                );
            }
        }
        next_frame().await
    }
}
