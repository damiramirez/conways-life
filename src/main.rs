mod conways;
mod six_seven;

use conways::Conways;
use macroquad::prelude::*;
use six_seven::SixSevenGame;

const CELL_SIZE: f32 = 20.;
const TICK_RATE: f32 = 1.0 / 11.;
const GRID_COLS: usize = 32;
const GRID_ROWS: usize = 32;

enum Mode {
    Conways,
    SixSeven(SixSevenGame),
}

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
    let mut mode = Mode::Conways;

    loop {
        let dt = get_frame_time();
        clear_background(BLACK);

        match &mut mode {
            Mode::Conways => {
                draw_conways_grid(&conways);
                draw_conways_hints();

                if is_mouse_button_pressed(MouseButton::Left) {
                    let (cell_x, cell_y) = mouse_cell();
                    conways.toggle_state_cell((cell_x, cell_y));
                }

                accum_delta += dt;
                if running && accum_delta >= TICK_RATE {
                    accum_delta -= TICK_RATE;
                    conways.update_cells();
                } else if !running {
                    accum_delta = 0.;
                    draw_centered_text("PAUSE", 40, RED);
                }

                if is_key_released(KeyCode::Space) {
                    running = !running;
                }

                if is_key_pressed(KeyCode::LeftShift) {
                    conways.kill_all_cells();
                }

                if is_key_pressed(KeyCode::Key6) {
                    mode = Mode::SixSeven(SixSevenGame::new(GRID_COLS, GRID_ROWS));
                }
            }
            Mode::SixSeven(game) => {
                draw_six_seven(game);

                if !game.game_over && is_mouse_button_pressed(MouseButton::Left) {
                    let (cell_x, cell_y) = mouse_cell();
                    game.click(cell_x, cell_y);
                }

                game.tick(dt);

                if game.game_over && is_key_pressed(KeyCode::R) {
                    game.restart();
                }

                if is_key_pressed(KeyCode::Escape) {
                    mode = Mode::Conways;
                    accum_delta = 0.;
                }
            }
        }

        next_frame().await
    }
}

fn mouse_cell() -> (usize, usize) {
    let (mouse_x, mouse_y) = mouse_position();
    (
        (mouse_x / CELL_SIZE).floor().max(0.) as usize,
        (mouse_y / CELL_SIZE).floor().max(0.) as usize,
    )
}

fn draw_conways_grid(conways: &Conways) {
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
}

fn draw_conways_hints() {
    draw_text(
        "Start/Stop game with SPACE",
        10.,
        screen_height() - CELL_SIZE * 5.5,
        28.,
        RED,
    );
    draw_text(
        "Click on the cells to create or delete one",
        10.,
        screen_height() - CELL_SIZE * 4.,
        28.,
        RED,
    );
    draw_text(
        "Kill all the cells with LEFT SHIFT",
        10.,
        screen_height() - CELL_SIZE * 2.5,
        28.,
        RED,
    );
    draw_text(
        "Press 6 to enter SIX SEVEN mode",
        10.,
        screen_height() - CELL_SIZE,
        28.,
        YELLOW,
    );
}

fn draw_six_seven(game: &SixSevenGame) {
    for (cell_x, cell_y) in game.digit_cells() {
        draw_rectangle(
            cell_x as f32 * CELL_SIZE,
            cell_y as f32 * CELL_SIZE,
            CELL_SIZE,
            CELL_SIZE,
            WHITE,
        );
    }

    let timer_ratio = (game.time_left / game.time_limit).clamp(0., 1.);
    let bar_w = screen_width() - 20.;
    draw_rectangle(10., 10., bar_w, 14., DARKGRAY);
    draw_rectangle(
        10.,
        10.,
        bar_w * timer_ratio,
        14.,
        if timer_ratio < 0.3 { RED } else { GREEN },
    );

    draw_text(
        &format!("SCORE: {}", game.score),
        10.,
        screen_height() - CELL_SIZE * 4.,
        32.,
        YELLOW,
    );
    draw_text(
        "Click the digit before the timer runs out!",
        10.,
        screen_height() - CELL_SIZE * 2.5,
        24.,
        RED,
    );
    draw_text(
        "ESC: back to Conway   R: restart on game over",
        10.,
        screen_height() - CELL_SIZE,
        24.,
        RED,
    );

    if game.game_over {
        draw_rectangle(
            0.,
            screen_height() / 2.0 - 80.,
            screen_width(),
            160.,
            Color::new(0., 0., 0., 0.85),
        );
        draw_centered_text("YOU AIN'T SIX SEVEN", 36, RED);
        let score_text = format!("FINAL SCORE: {}", game.score);
        let dim = measure_text(&score_text, None, 32, 1.);
        draw_text(
            &score_text,
            (screen_width() - dim.width) / 2.0,
            screen_height() / 2.0 + 20.,
            32.,
            WHITE,
        );
        let hint = "Press R to try again";
        let dim = measure_text(hint, None, 24, 1.);
        draw_text(
            hint,
            (screen_width() - dim.width) / 2.0,
            screen_height() / 2.0 + 55.,
            24.,
            YELLOW,
        );
    }
}

fn draw_centered_text(text: &str, font_size: u16, color: Color) {
    let dim = measure_text(text, None, font_size, 1.);
    draw_text(
        text,
        (screen_width() - dim.width) / 2.0,
        (screen_height() - dim.height) / 2.0,
        font_size as f32,
        color,
    );
}
