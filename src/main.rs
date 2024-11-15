mod conways;

use conways::Conways;
use macroquad::prelude::*;

const CELL_SIZE: f32 = 20.;
const UPDATE_TIMER: f64 = 0.1;

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
    let mut last_updated = 0_f64;
    let mut conways = Conways::from_random_cells();

    loop {
        clear_background(BLACK);

        for (x_row, row) in conways.grid.iter().enumerate() {
            for (y_col, col) in row.iter().enumerate() {
                draw_rectangle(
                    x_row as f32 * CELL_SIZE,
                    y_col as f32 * CELL_SIZE,
                    CELL_SIZE,
                    CELL_SIZE,
                    match col {
                        conways::CellState::Alive => WHITE,
                        conways::CellState::Dead => BLACK,
                    },
                );
            }
        }

        if get_time() - last_updated > UPDATE_TIMER {
            last_updated = get_time();
            conways.update_cells();
        }

        next_frame().await
    }
}
