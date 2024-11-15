mod conways;

use conways::Conways;
use macroquad::prelude::*;

const CELL_SIZE: f32 = 16.;
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
        (1, 3),
        (1, 5),
        (2, 1),
        (2, 7),
        (3, 3),
        (3, 6),
        (4, 2),
        (4, 4),
        (4, 8),
        (5, 1),
        (5, 5),
        (5, 7),
        (6, 3),
        (6, 4),
        (6, 8),
        (7, 2),
        (7, 5),
    ];
    let mut conways: Conways = Conways::from(alive_cells);

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
