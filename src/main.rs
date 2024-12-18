mod conways;

use conways::Conways;
use macroquad::prelude::*;

const CELL_SIZE: f32 = 20.;
const TICK_RATE: f32 = 1.0 / 11.;

fn conf() -> Conf {
    Conf {
        window_title: "Conway's Game of Life".to_string(),
        window_height: 800,
        window_width: 640,
        fullscreen: false,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(conf)]
async fn main() {
    let mut conways = Conways::from_random_cells();
    let mut running = true;
    let mut accum_delta: f32 = 0.;
    loop {
        accum_delta += get_frame_time();
        clear_background(BLACK);

        for (x_row, row) in conways.grid.iter().enumerate() {
            for (y_col, col) in row.iter().enumerate() {
                let color = match col {
                    conways::CellState::Alive => WHITE,
                    conways::CellState::Dead => BLACK,
                };

                draw_rectangle(
                    x_row as f32 * CELL_SIZE,
                    y_col as f32 * CELL_SIZE,
                    CELL_SIZE,
                    CELL_SIZE,
                    color,
                );
            }
        }

        draw_text(
            "Start/Stop game with SPACE",
            10.,
            screen_height() - CELL_SIZE * 4.,
            32.,
            RED,
        );

        draw_text(
            "Click on the cells to create or delete one",
            10.,
            screen_height() - CELL_SIZE * 2.5,
            32.,
            RED,
        );

        draw_text(
            "Kill all the cells with LEFT SHIFT",
            10.,
            screen_height() - CELL_SIZE,
            32.,
            RED,
        );

        if is_mouse_button_pressed(MouseButton::Left) {
            let (mouse_x, mouse_y) = mouse_position();
            let (mark_cell_x, mark_cell_y): (usize, usize) = (
                (mouse_x / CELL_SIZE).floor() as usize,
                (mouse_y / CELL_SIZE).floor() as usize,
            );
            draw_rectangle(
                mark_cell_x as f32 * CELL_SIZE,
                mark_cell_y as f32 * CELL_SIZE,
                CELL_SIZE,
                CELL_SIZE,
                WHITE,
            );

            conways.toggle_state_cell((mark_cell_x, mark_cell_y));
        }

        if running && accum_delta >= TICK_RATE {
            accum_delta -= TICK_RATE;
            conways.update_cells();
        } else if !running {
            accum_delta = 0.;
            let dimension = measure_text("PAUSE", None, 40, 1.);
            draw_text(
                "PAUSE",
                (screen_width() - dimension.width) / 2.0,
                (screen_height() - dimension.height) / 2.0,
                40.,
                RED,
            );
        }

        if is_key_released(KeyCode::Space) {
            running = !running;
        }

        if is_key_pressed(KeyCode::LeftShift) {
            conways.kill_all_cells();
        }

        next_frame().await
    }
}
