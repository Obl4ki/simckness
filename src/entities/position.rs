use crate::constants::N_CELLS;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}
