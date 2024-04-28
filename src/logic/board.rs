use nalgebra::DMatrix;

use super::{error::GameError, insertion::InsertionDirection};

pub struct GameBoard {
    pub board: DMatrix<u8>,
}

impl GameBoard {
    pub fn new(n: usize) -> Self {
        Self {
            board: DMatrix::zeros(n, n),
        }
    }

    pub fn place(&mut self, slot: usize) -> Result<(), GameError> {
        let insertion_direction = InsertionDirection::for_board_insertion(&self.board, slot)?;

        Ok(())
    }
}
