use nalgebra::{DMatrix, RowDVector};

use super::{error::GameError, insertion::InsertionDirection};

pub struct GameBoard {
    board: DMatrix<u8>,
}

impl GameBoard {
    pub fn new(n: usize) -> Self {
        Self {
            board: DMatrix::zeros(n, n),
        }
    }

    pub fn board(&self) -> &DMatrix<u8> {
        &self.board
    }

    pub fn place(&mut self, slot: usize) -> Result<(), GameError> {
        let insertion_direction = InsertionDirection::for_board_insertion(&self.board, slot)?;
        let index = insertion_direction.get_side_index(&self.board, slot);

        match insertion_direction {
            InsertionDirection::FromTop | InsertionDirection::FromBottom => {
                let mut column = self.board.column_mut(index);
                let slice = column.as_mut_slice();

                insertion_direction.place(slice)?;
            }
            InsertionDirection::FromRight | InsertionDirection::FromLeft => {
                let row = self.board.row(index);
                let mut data = row.iter().map(|&x| x).collect::<Vec<_>>();

                insertion_direction.place(&mut data)?;

                self.board.set_row(
                    index,
                    &RowDVector::from_row_iterator(data.len(), data.into_iter()),
                );
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use nalgebra::{DMatrix, RowDVector};

    use super::GameBoard;

    #[test]
    pub fn verify_place_top() {
        let mut game_board = GameBoard::new(3);

        // top place
        game_board.place(0).unwrap();

        assert_eq!(
            game_board.board(),
            &DMatrix::from_rows(&[
                RowDVector::from_vec(vec![0, 0, 0]),
                RowDVector::from_vec(vec![0, 0, 0]),
                RowDVector::from_vec(vec![1, 0, 0]),
            ])
        );
    }

    #[test]
    pub fn verify_place_right() {
        let mut game_board = GameBoard::new(3);

        // top place
        game_board.place(3).unwrap();

        assert_eq!(
            game_board.board(),
            &DMatrix::from_rows(&[
                RowDVector::from_vec(vec![1, 0, 0]),
                RowDVector::from_vec(vec![0, 0, 0]),
                RowDVector::from_vec(vec![0, 0, 0]),
            ])
        );
    }

    #[test]
    pub fn verify_place_bottom() {
        let mut game_board = GameBoard::new(3);

        // top place
        game_board.place(8).unwrap();

        assert_eq!(
            game_board.board(),
            &DMatrix::from_rows(&[
                RowDVector::from_vec(vec![1, 0, 0]),
                RowDVector::from_vec(vec![0, 0, 0]),
                RowDVector::from_vec(vec![0, 0, 0]),
            ])
        );
    }

    #[test]
    pub fn verify_place_left() {
        let mut game_board = GameBoard::new(3);

        // top place
        game_board.place(11).unwrap();

        assert_eq!(
            game_board.board(),
            &DMatrix::from_rows(&[
                RowDVector::from_vec(vec![0, 0, 1]),
                RowDVector::from_vec(vec![0, 0, 0]),
                RowDVector::from_vec(vec![0, 0, 0]),
            ])
        );
    }

    #[test]
    pub fn verify_place_stacking_1() {
        let mut game_board = GameBoard::new(3);

        // top place
        game_board.place(0).unwrap();
        game_board.place(0).unwrap();
        game_board.place(4).unwrap();

        assert_eq!(
            game_board.board(),
            &DMatrix::from_rows(&[
                RowDVector::from_vec(vec![0, 0, 0]),
                RowDVector::from_vec(vec![1, 1, 0]),
                RowDVector::from_vec(vec![1, 0, 0]),
            ])
        );
    }

    #[test]
    pub fn verify_place_stacking_2() {
        let mut game_board = GameBoard::new(3);

        // top place
        game_board.place(0).unwrap();
        game_board.place(9).unwrap();
        game_board.place(8).unwrap();

        assert_eq!(
            game_board.board(),
            &DMatrix::from_rows(&[
                RowDVector::from_vec(vec![0, 0, 0]),
                RowDVector::from_vec(vec![1, 0, 0]),
                RowDVector::from_vec(vec![1, 1, 0]),
            ])
        );
    }
}
