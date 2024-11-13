mod grid;

use crate::grid::{CellState, Grid};
use macroquad::prelude::*;

const SLOW_SPEED: f64 = 0.3;
const FAST_SPEED: f64 = 0.05;

fn window_conf() -> Conf {
    Conf {
        window_title: String::from("Conways"),
        window_width: 1024,
        window_height: 1024,
        fullscreen: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut last_update = get_time();
    let mut speed = SLOW_SPEED;
    let mut grid: Grid = grid::create_grid(vec![
        (5, 5),
        (6, 6),
        (7, 4),
        (7, 5),
        (7, 6), // Glider
        (12, 12),
        (13, 12),
        (14, 12), // Stick
    ]);

    let mut running = true;
    while running {
        clear_background(DARKGRAY);

        if is_key_pressed(KeyCode::Space) {
            speed = FAST_SPEED;
        } else if is_key_released(KeyCode::Space) {
            speed = SLOW_SPEED;
        }

        if is_key_pressed(KeyCode::N) {
            grid = grid::create_random_grid()
        }

        if is_key_pressed(KeyCode::Escape) {
            running = false;
        }

        if get_time() - last_update > speed {
            last_update = get_time();
            grid = grid::next_state_for_grid(&grid);
        }

        for (i, row) in grid.iter().enumerate() {
            for (j, _) in row.iter().enumerate() {
                let state = grid::get_cell_state(&grid, i, j).unwrap_or_default();
                let color = if state == CellState::Alive {
                    YELLOW
                } else {
                    LIGHTGRAY
                };
                draw_rectangle(
                    (grid::GRID_WIDTH * i) as f32,
                    (grid::GRID_HEIGHT * j) as f32,
                    grid::CELL_SIZE,
                    grid::CELL_SIZE,
                    color,
                );
            }
        }

        draw_text("Hold SPACE to advance speed", 20.0, 20.0, 30.0, RED);
        draw_text("Press N to randomize", 430.0, 20.0, 30.0, RED);
        draw_text("Press ESC to exit", 800.0, 20.0, 30.0, RED);
        next_frame().await
    }
}
