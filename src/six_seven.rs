use rand::Rng;

const DIGIT_W: usize = 10;
const DIGIT_H: usize = 14;

const DIGIT_6: [&[u8; DIGIT_W]; DIGIT_H] = [
    b"..######..",
    b".########.",
    b"##......##",
    b"##........",
    b"##........",
    b"##........",
    b"##........",
    b"########..",
    b"##......##",
    b"##......##",
    b"##......##",
    b"##......##",
    b".########.",
    b"..######..",
];

const DIGIT_7: [&[u8; DIGIT_W]; DIGIT_H] = [
    b"##########",
    b"##########",
    b"........##",
    b"........##",
    b".......##.",
    b".......##.",
    b"......##..",
    b"......##..",
    b".....##...",
    b".....##...",
    b"....##....",
    b"....##....",
    b"...##.....",
    b"...##.....",
];

const INITIAL_TIME: f32 = 2.0;
const MIN_TIME: f32 = 0.4;
const TIME_STEP: f32 = 0.06;

pub struct SixSevenGame {
    pub digit: u8,
    pub time_left: f32,
    pub time_limit: f32,
    pub score: u32,
    pub game_over: bool,
    pub offset_x: usize,
    pub offset_y: usize,
    pub grid_cols: usize,
    pub grid_rows: usize,
}

impl SixSevenGame {
    pub fn new(grid_cols: usize, grid_rows: usize) -> Self {
        let mut game = Self {
            digit: 6,
            time_left: INITIAL_TIME,
            time_limit: INITIAL_TIME,
            score: 0,
            game_over: false,
            offset_x: 0,
            offset_y: 0,
            grid_cols,
            grid_rows,
        };
        game.place_digit();
        game
    }

    fn place_digit(&mut self) {
        let max_x = self.grid_cols.saturating_sub(DIGIT_W);
        let max_y = self.grid_rows.saturating_sub(DIGIT_H);
        let mut rng = rand::thread_rng();

        // Start centered; randomize position as score grows.
        if self.score < 3 {
            self.offset_x = max_x / 2;
            self.offset_y = max_y / 2;
        } else {
            self.offset_x = rng.gen_range(0..=max_x);
            self.offset_y = rng.gen_range(0..=max_y);
        }
    }

    pub fn tick(&mut self, dt: f32) {
        if self.game_over {
            return;
        }
        self.time_left -= dt;
        if self.time_left <= 0. {
            self.game_over = true;
        }
    }

    pub fn click(&mut self, cell_x: usize, cell_y: usize) {
        if self.game_over {
            return;
        }
        if self.is_digit_cell(cell_x, cell_y) {
            self.score += 1;
            self.digit = if self.digit == 6 { 7 } else { 6 };
            self.time_limit = (self.time_limit - TIME_STEP).max(MIN_TIME);
            self.time_left = self.time_limit;
            self.place_digit();
        } else {
            self.game_over = true;
        }
    }

    pub fn restart(&mut self) {
        *self = Self::new(self.grid_cols, self.grid_rows);
    }

    fn is_digit_cell(&self, cell_x: usize, cell_y: usize) -> bool {
        if cell_x < self.offset_x || cell_y < self.offset_y {
            return false;
        }
        let local_x = cell_x - self.offset_x;
        let local_y = cell_y - self.offset_y;
        if local_x >= DIGIT_W || local_y >= DIGIT_H {
            return false;
        }
        self.current_bitmap()[local_y][local_x] == b'#'
    }

    fn current_bitmap(&self) -> &'static [&'static [u8; DIGIT_W]; DIGIT_H] {
        if self.digit == 6 {
            &DIGIT_6
        } else {
            &DIGIT_7
        }
    }

    pub fn digit_cells(&self) -> impl Iterator<Item = (usize, usize)> + '_ {
        let bitmap = self.current_bitmap();
        (0..DIGIT_H).flat_map(move |y| {
            (0..DIGIT_W).filter_map(move |x| {
                if bitmap[y][x] == b'#' {
                    Some((self.offset_x + x, self.offset_y + y))
                } else {
                    None
                }
            })
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn click_on_digit_scores() {
        let mut game = SixSevenGame::new(32, 32);
        let (x, y) = game.digit_cells().next().unwrap();
        game.click(x, y);
        assert_eq!(game.score, 1);
        assert!(!game.game_over);
        assert_eq!(game.digit, 7);
    }

    #[test]
    fn click_outside_digit_ends_game() {
        let mut game = SixSevenGame::new(32, 32);
        game.click(0, 0);
        assert!(game.game_over);
        assert_eq!(game.score, 0);
    }

    #[test]
    fn timer_runs_out() {
        let mut game = SixSevenGame::new(32, 32);
        game.tick(INITIAL_TIME + 0.1);
        assert!(game.game_over);
    }

    #[test]
    fn difficulty_increases() {
        let mut game = SixSevenGame::new(32, 32);
        let initial_limit = game.time_limit;
        let (x, y) = game.digit_cells().next().unwrap();
        game.click(x, y);
        assert!(game.time_limit < initial_limit);
    }
}
