mod conways;

use conways::Conways;
use macroquad::prelude::*;

const CELL_SIZE: f32 = 20.;
const UPDATE_TIMER: f64 = 0.1;

fn conf() -> Conf {
    Conf {
        window_title: "Conway's Game of Life".to_string(),
        window_height: 740,
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

    let mut running = true;
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

        draw_text(
            "Start/Stop game with SPACE",
            10.,
            screen_height() - CELL_SIZE * 2.5,
            32.,
            RED,
        );

        draw_text(
            "Click on the cells to create or delete one",
            10.,
            screen_height() - CELL_SIZE,
            32.,
            RED,
        );

        if is_mouse_button_pressed(MouseButton::Left) {
            let (mouse_x, mouse_y) = mouse_position();
            let mark_cell: (usize, usize) = (
                (mouse_x / CELL_SIZE).floor() as usize,
                (mouse_y / CELL_SIZE).floor() as usize,
            );
            draw_rectangle(
                mark_cell.0 as f32 * CELL_SIZE,
                mark_cell.1 as f32 * CELL_SIZE,
                CELL_SIZE,
                CELL_SIZE,
                WHITE,
            );

            conways.update_state_cell(mark_cell);
        }

        if running && get_time() - last_updated > UPDATE_TIMER {
            last_updated = get_time();
            conways.update_cells();
        }

        if is_key_released(KeyCode::Space) {
            running = !running;
        }

        next_frame().await
    }
}
