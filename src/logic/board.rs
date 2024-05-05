use bevy::log::info;
use nalgebra::{DMatrix, RowDVector};

use super::{error::GameError, insertion::InsertionDirection};

#[derive(Clone)]
pub struct GameBoard {
    board: DMatrix<u8>,
    offset: i8,
    display_board: DMatrix<u8>,
}

impl GameBoard {
    pub fn new(n: usize) -> Self {
        Self {
            board: DMatrix::zeros(n, n),
            offset: 0,
            display_board: DMatrix::zeros(n, n),
        }
    }

    pub fn board(&self) -> &DMatrix<u8> {
        &self.board
    }

    pub fn display_board(&self) -> &DMatrix<u8> {
        &self.display_board
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

        self.update_display_board(0);

        Ok(())
    }

    fn rotate_board_right(mut board: DMatrix<u8>) -> DMatrix<u8> {
        board.transpose_mut();

        let width = board.ncols();
        let half_width = width / 2;

        (0..half_width).for_each(|i| board.swap_columns(i, width - i - 1));

        board
    }

    pub fn rotate_right(&mut self) {
        self.board = Self::rotate_board_right(self.board.clone());
        self.update_display_board(1);
    }

    fn rotate_board_left(mut board: DMatrix<u8>) -> DMatrix<u8> {
        let width = board.ncols();
        let half_width = width / 2;

        (0..half_width).for_each(|i| board.swap_columns(i, width - i - 1));

        board.transpose_mut();

        board
    }

    pub fn rotate_left(&mut self) {
        self.board = Self::rotate_board_left(self.board.clone());
        self.update_display_board(-1);
    }

    fn update_display_board(&mut self, change: i8) {
        self.offset += change;

        let mut new_board = self.board.clone();

        if self.offset.abs() > 0 {
            let range = if self.offset > 0 {
                0..self.offset
            } else {
                self.offset..0
            };

            for r in range.into_iter() {
                if r >= 0 {
                    new_board = Self::rotate_board_left(new_board);
                } else {
                    new_board = Self::rotate_board_right(new_board);
                }
            }
        }

        self.display_board = new_board;
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

    #[test]
    pub fn verify_rotate_right() {
        let mut game_board = GameBoard::new(3);

        game_board.place(1).unwrap();
        game_board.place(2).unwrap();
        game_board.place(3).unwrap();

        assert_eq!(
            game_board.board(),
            &DMatrix::from_rows(&[
                RowDVector::from_vec(vec![1, 0, 0]),
                RowDVector::from_vec(vec![0, 0, 0]),
                RowDVector::from_vec(vec![0, 1, 1]),
            ])
        );

        game_board.rotate_right();

        assert_eq!(
            game_board.board(),
            &DMatrix::from_rows(&[
                RowDVector::from_vec(vec![0, 0, 1]),
                RowDVector::from_vec(vec![1, 0, 0]),
                RowDVector::from_vec(vec![1, 0, 0]),
            ])
        );
    }

    #[test]
    pub fn verify_rotate_right_large() {
        let mut game_board = GameBoard::new(5);

        game_board.place(1).unwrap();
        game_board.place(2).unwrap();
        game_board.place(3).unwrap();
        game_board.place(3).unwrap();

        assert_eq!(
            game_board.board(),
            &DMatrix::from_rows(&[
                RowDVector::from_vec(vec![0, 0, 0, 0, 0]),
                RowDVector::from_vec(vec![0, 0, 0, 0, 0]),
                RowDVector::from_vec(vec![0, 0, 0, 0, 0]),
                RowDVector::from_vec(vec![0, 0, 0, 1, 0]),
                RowDVector::from_vec(vec![0, 1, 1, 1, 0]),
            ])
        );

        game_board.rotate_right();

        assert_eq!(
            game_board.board(),
            &DMatrix::from_rows(&[
                RowDVector::from_vec(vec![0, 0, 0, 0, 0]),
                RowDVector::from_vec(vec![1, 0, 0, 0, 0]),
                RowDVector::from_vec(vec![1, 0, 0, 0, 0]),
                RowDVector::from_vec(vec![1, 1, 0, 0, 0]),
                RowDVector::from_vec(vec![0, 0, 0, 0, 0]),
            ])
        );
    }

    #[test]
    pub fn verify_rotate_left() {
        let mut game_board = GameBoard::new(3);

        game_board.place(1).unwrap();
        game_board.place(2).unwrap();
        game_board.place(3).unwrap();

        assert_eq!(
            game_board.board(),
            &DMatrix::from_rows(&[
                RowDVector::from_vec(vec![1, 0, 0]),
                RowDVector::from_vec(vec![0, 0, 0]),
                RowDVector::from_vec(vec![0, 1, 1]),
            ])
        );

        game_board.rotate_left();

        assert_eq!(
            game_board.board(),
            &DMatrix::from_rows(&[
                RowDVector::from_vec(vec![0, 0, 1]),
                RowDVector::from_vec(vec![0, 0, 1]),
                RowDVector::from_vec(vec![1, 0, 0]),
            ])
        );
    }

    #[test]
    pub fn verify_display_board() {
        let mut board = GameBoard::new(3);
        board.place(0).unwrap();

        assert_eq!(board.board(), board.display_board());

        let upright_position = board.clone();

        for i in (-10..20).into_iter() {
            if i < 0 {
                board.rotate_left();
            } else {
                board.rotate_right();
            }

            // upright position board should be the same for the board's display board
            assert_eq!(upright_position.board(), board.display_board());
        }
    }

    #[test]
    pub fn verify_corner_case() {
        let mut game_board = GameBoard::new(3);

        game_board.place(3).unwrap();
        game_board.place(3).unwrap();
        game_board.place(4).unwrap();

        assert_eq!(
            game_board.board(),
            &DMatrix::from_rows(&[
                RowDVector::from_vec(vec![1, 1, 0]),
                RowDVector::from_vec(vec![1, 0, 0]),
                RowDVector::from_vec(vec![0, 0, 0]),
            ])
        );

        game_board.place(0).unwrap();

        assert_eq!(
            game_board.board(),
            &DMatrix::from_rows(&[
                RowDVector::from_vec(vec![1, 1, 0]),
                RowDVector::from_vec(vec![1, 0, 0]),
                RowDVector::from_vec(vec![1, 0, 0]),
            ])
        );
    }

    #[test]
    pub fn verify_corner_case_2() {
        let mut game_board = GameBoard::new(3);

        game_board.place(4).unwrap();

        assert_eq!(
            game_board.board(),
            &DMatrix::from_rows(&[
                RowDVector::from_vec(vec![0, 0, 0]),
                RowDVector::from_vec(vec![1, 0, 0]),
                RowDVector::from_vec(vec![0, 0, 0]),
            ])
        );

        game_board.place(0).unwrap();

        assert_eq!(
            game_board.board(),
            &DMatrix::from_rows(&[
                RowDVector::from_vec(vec![0, 0, 0]),
                RowDVector::from_vec(vec![1, 0, 0]),
                RowDVector::from_vec(vec![1, 0, 0]),
            ])
        );

        game_board.place(0).unwrap();

        assert_eq!(
            game_board.board(),
            &DMatrix::from_rows(&[
                RowDVector::from_vec(vec![1, 0, 0]),
                RowDVector::from_vec(vec![1, 0, 0]),
                RowDVector::from_vec(vec![1, 0, 0]),
            ])
        );
    }

    #[test]
    pub fn verify_corner_case_3() {
        let mut game_board = GameBoard::new(4);

        [0, 6, 5, 8, 12, 15, 14, 6, 3, 14, 4, 6, 6, 11]
            .into_iter()
            .for_each(|place| {
                println!("Placing {place}");
                game_board.place(place).unwrap();
                println!("Board {}", game_board.board);
            });

        println!("{}", game_board.board);

        let result = game_board.place(11);

        assert_eq!(result, Ok(()));

        println!("{}", game_board.board);
    }
}
