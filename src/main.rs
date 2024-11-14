mod conways;

use macroquad::prelude::*;
use conways::Conways;

const SIZE: f32 = 16.;
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

    let alive_cells = vec![
        conways::Position(1, 3),
        conways::Position(1, 5),
        conways::Position(2, 1),
        conways::Position(2, 7),
        conways::Position(3, 3),
        conways::Position(3, 6),
        conways::Position(4, 2),
        conways::Position(4, 4),
        conways::Position(4, 8),
        conways::Position(5, 1),
        conways::Position(5, 5),
        conways::Position(5, 7),
        conways::Position(6, 3),
        conways::Position(6, 4),
        conways::Position(6, 8),
        conways::Position(7, 2),
        conways::Position(7, 5),
    ];
    let mut conways: Conways = Conways::from(alive_cells);

    loop {
        clear_background(BLACK);

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
                    }
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