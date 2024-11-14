mod conways;

use macroquad::prelude::*;
use conways::Conways;

const SIZE: f32 = 16.;

fn conf() -> Conf {
    Conf {
        window_title: "Conway's Game of Life".to_string(),
        window_height: 640,
        window_width: 640,
        fullscreen: false,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(conf)]
async fn main() {
    loop {
        clear_background(BLACK);

        let conways: Conways = Conways::new();
        for (x_row, row) in conways.grid.iter().enumerate() {
            for (y_col, col) in row.iter().enumerate() {
                draw_rectangle(
                    x_row as f32 * SIZE,
                    y_col as f32 * SIZE,
                    SIZE,
                    SIZE,
                    match col {
                        conways::CellState::Alive => WHITE,
                        conways::CellState::Dead => BLACK,
                    },);
            }
        }

        next_frame().await
    }
}