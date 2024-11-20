mod grid;

use crate::grid::{cell_is_alive, dimensions, Grid};
use macroquad::prelude::*;

const SLOW_SPEED: f64 = 0.3;
const FAST_SPEED: f64 = 0.05;

const DEFAULT_GRID_WIDTH: usize = 32;
const DEFAULT_GRID_HEIGHT: usize = 32;

const SCREEN_WIDTH: i32 = 1024;
const SCREEN_HEIGHT: i32 = 1024;

fn window_conf() -> Conf {
    Conf {
        window_title: String::from("Conways"),
        window_width: SCREEN_WIDTH,
        window_height: SCREEN_HEIGHT,
        fullscreen: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut last_update = get_time();
    let mut speed = SLOW_SPEED;
    let mut grid: Grid = grid::create_grid(
        DEFAULT_GRID_WIDTH,
        DEFAULT_GRID_HEIGHT,
        vec![
            (5, 5),
            (6, 6),
            (7, 4),
            (7, 5),
            (7, 6), // Glider
            (12, 12),
            (13, 12),
            (14, 12), // Stick
        ],
    );

    let mut running = true;
    while running {
        // Handle Input
        if is_key_pressed(KeyCode::Space) {
            speed = FAST_SPEED;
        } else if is_key_released(KeyCode::Space) {
            speed = SLOW_SPEED;
        }

        if is_key_pressed(KeyCode::Escape) {
            running = false;
        }

        // Check if we should redraw/recalculate grid
        if get_time() - last_update < speed {
            continue;
        }
        last_update = get_time();

        let (grid_height, grid_width) = dimensions(&grid);
        if is_key_pressed(KeyCode::N) {
            grid = grid::create_random_grid((grid_width - 1).max(6), (grid_height - 1).max(6))
        } else if is_key_pressed(KeyCode::M) {
            grid = grid::create_random_grid(grid_width + 1, grid_height + 1)
        }

        grid = grid::next_state_for_grid(&grid);
        clear_background(DARKGRAY);
        let cell_width = SCREEN_WIDTH as f32 / grid_width as f32;
        let cell_height = SCREEN_HEIGHT as f32 / grid_height as f32;
        let cell_size = cell_width.min(cell_height);
        for (i, row) in grid.iter().enumerate() {
            for (j, _) in row.iter().enumerate() {
                let color = if cell_is_alive(&grid, i, j) {
                    YELLOW
                } else {
                    LIGHTGRAY
                };
                draw_rectangle(
                    cell_width * i as f32,
                    cell_width * j as f32,
                    cell_size,
                    cell_size,
                    color,
                );
            }
        }

        draw_text("Hold SPACE to advance speed", 20.0, 20.0, 30.0, RED);
        draw_text("Press N/M to randomize", 430.0, 20.0, 30.0, RED);
        draw_text("Press ESC to exit", 800.0, 20.0, 30.0, RED);
        next_frame().await
    }
}
