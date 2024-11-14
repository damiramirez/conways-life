const COLUMNS: usize = 40;
const ROW: usize = 40;

#[derive(Clone, Debug)]
pub enum CellState {
  Alive,
  Dead
}

pub struct Conways {
  pub grid: Vec<Vec<CellState>>
}

impl Conways {
    pub fn new() -> Self {
      Self {
        grid: vec![vec![CellState::Dead; ROW]; COLUMNS]
      }
    }
}