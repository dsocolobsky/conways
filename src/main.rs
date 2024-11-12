use macroquad::prelude::*;

#[macroquad::main("Conways")]
async fn main() {
    loop {
        clear_background(LIGHTGRAY);
        draw_text("Conways Game of Life", 20.0, 20.0, 30.0, BLACK);
        next_frame().await
    }
}
