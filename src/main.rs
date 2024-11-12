use macroquad::prelude::*;

const GRID_WIDTH: usize = 32;
const GRID_HEIGHT: usize = 32;
const CELL_SIZE: f32 = 30.0;

#[derive(Copy, Clone, PartialEq)]
enum CellState {
    Alive,
    Dead,
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
    x >= 0 && x < GRID_WIDTH && y >= 0 && y < GRID_HEIGHT
}

fn get_state(grid: &Grid, x: usize, y: usize) -> CellState {
    if !valid_coordinate(x, y) {
        panic!("Invalid coordinate");
    }
    grid[x][y]
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

#[macroquad::main(window_conf)]
async fn main() {
    let mut grid: Grid = [[CellState::Dead; GRID_WIDTH]; GRID_HEIGHT];
    set_alive(&mut grid, 4, 4);

    loop {
        clear_background(DARKGRAY);

        for i in (0..GRID_WIDTH) {
            for j in (0..GRID_HEIGHT) {
                let state = get_state(&grid, i, j);
                let color = if (state == CellState::Alive) {
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
