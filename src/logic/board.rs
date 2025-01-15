use bevy::log::debug;
use nalgebra::{DMatrix, RowDVector};

use super::{error::GameError, insertion::InsertionDirection, score::GameScore};

#[derive(Debug, Clone)]
pub struct GameBoard {
    /// Playing Board
    board: DMatrix<u8>,
    /// Queued Board Rotations, every increment is a 90 degree turn.
    /// Positive is clockwise, negative, is counter-clockwise.
    ///
    /// This is cleared when preparing the next drop.
    offset: i8,
    /// A copy of the playing board with offset operations reversed.
    /// This should be used when displaying the board graphically and applying the rotations
    /// via graphical transformations (like rotating game pieces)
    display_board: DMatrix<u8>,
    /// Enables row/col clearing
    rows_clearing: bool,
    /// The current score on the board
    score: GameScore,
}

impl GameBoard {
    /// Constructs a square game board of `n` size on each side
    pub fn new(n: usize) -> Self {
        Self {
            board: DMatrix::zeros(n, n),
            offset: 0,
            display_board: DMatrix::zeros(n, n),
            rows_clearing: false,
            score: GameScore::new(),
        }
    }

    /// Enables row clearing on the game board
    pub fn with_rows_clearing(mut self) -> Self {
        self.rows_clearing = true;
        self
    }

    /// The `n` dimension of the board
    pub fn width(&self) -> usize {
        self.board.ncols()
    }

    /// The current game board. This has all rotation logic applied through it
    /// as the game is played.
    pub fn board(&self) -> &DMatrix<u8> {
        &self.board
    }

    /// The game board to be displayed. This has no rotation logic applied to it.
    pub fn display_board(&self) -> &DMatrix<u8> {
        &self.display_board
    }

    /// The current score
    pub fn score(&self) -> &GameScore {
        &self.score
    }

    /// Returns the coordinate where the tile was placed (column, row)
    pub fn place(&mut self, slot: usize) -> Result<(usize, usize), GameError> {
        let insertion_direction = InsertionDirection::for_board_insertion(&self.board, slot)?;
        debug!("Dropping into {slot} ({:?})", insertion_direction);
        let index = insertion_direction.get_side_index(&self.board, slot);

        let pos = match insertion_direction {
            InsertionDirection::FromTop => {
                let mut column = self.board.column_mut(index);
                let slice = column.as_mut_slice();

                (index, insertion_direction.place(slice)?)
            }
            InsertionDirection::FromLeft => {
                let row = self.board.row(index);
                let mut data = row.iter().map(|&x| x).collect::<Vec<_>>();

                let res = insertion_direction.place(&mut data)?;

                self.board.set_row(
                    index,
                    &RowDVector::from_row_iterator(data.len(), data.into_iter()),
                );

                (res, index)
            }
            InsertionDirection::FromRight => {
                let row = self.board.row(index);
                let mut data = row.iter().rev().map(|&x| x).collect::<Vec<_>>();

                let res = insertion_direction.place(&mut data)?;

                self.board.set_row(
                    index,
                    &RowDVector::from_row_iterator(data.len(), data.into_iter().rev()),
                );

                (res, index)
            }
            InsertionDirection::FromBottom => {
                let column = self.board.column_mut(index);
                let mut data = column.iter().rev().map(|&x| x).collect::<Vec<_>>();

                let len = data.len();

                let res = insertion_direction.place(&mut data)?;

                data.reverse();

                self.board.set_column(index, &data.into());

                (index, len.saturating_sub(res))
            }
        };

        self.score.add_score(1);

        if self.rows_clearing {
            self.check_full_rows(insertion_direction, index);
        }

        self.score.reset_drop_timer();

        self.update_display_board(0);

        Ok(pos)
    }

    /// Returns whether a row is full.
    fn check_row(&self, insertion_direction: InsertionDirection, index: usize) -> bool {
        match insertion_direction {
            InsertionDirection::FromTop | InsertionDirection::FromBottom => {
                let data = self.board.column(index);
                data.into_iter().all(|&x| x == 1)
            }
            InsertionDirection::FromRight | InsertionDirection::FromLeft => {
                let data = self.board.row(index);
                data.into_iter().all(|&x| x == 1)
            }
        }
    }

    /// Determines if any rows or colmns are full, and then scores (and clear if enabled).
    fn check_full_rows(&mut self, insertion_direction: InsertionDirection, index: usize) {
        let mut rows = Vec::new();
        let mut cols = Vec::new();

        match insertion_direction {
            InsertionDirection::FromTop | InsertionDirection::FromBottom => {
                let data = self.board.column(index);
                let slice_full = data.into_iter().all(|&x| x == 1);

                if slice_full {
                    cols.push(index);
                }

                (0..self.board.nrows())
                    .filter(|&index| self.check_row(InsertionDirection::FromLeft, index))
                    .for_each(|index| {
                        rows.push(index);
                    });
            }
            InsertionDirection::FromRight | InsertionDirection::FromLeft => {
                let data = self.board.row(index);
                let slice_full = data.into_iter().all(|&x| x == 1);

                if slice_full {
                    rows.push(index);
                }

                (0..self.board.ncols())
                    .filter(|&index| self.check_row(InsertionDirection::FromTop, index))
                    .for_each(|index| {
                        cols.push(index);
                    });
            }
        }

        let dim = self.board.ncols();
        let total_cleared = rows.len() + cols.len();

        if total_cleared > 0 {
            self.score.add_mult(total_cleared as u32);
            self.score.add_score((total_cleared * dim) as u32);
        }

        for index in rows {
            self.board.set_row(
                index,
                &RowDVector::from_row_iterator(
                    self.board.nrows(),
                    (0..self.board.nrows()).map(|_| 0),
                ),
            );
        }

        for index in cols {
            self.board
                .set_column(index, &vec![0_u8; self.board.ncols()].into());
        }
    }

    /// Rotates the board to the clockwise logically
    fn rotate_board_right(mut board: DMatrix<u8>) -> DMatrix<u8> {
        board.transpose_mut();

        let width = board.ncols();
        let half_width = width / 2;

        (0..half_width).for_each(|i| board.swap_columns(i, width - i - 1));

        board
    }

    /// Rotates the board clockwise
    pub fn rotate_right(&mut self) {
        self.board = Self::rotate_board_right(self.board.clone());
        self.update_display_board(1);
    }

    /// Rotates the board to the counter-clockwise logically
    fn rotate_board_left(mut board: DMatrix<u8>) -> DMatrix<u8> {
        let width = board.ncols();
        let half_width = width / 2;

        (0..half_width).for_each(|i| board.swap_columns(i, width - i - 1));

        board.transpose_mut();

        board
    }

    /// Rotates the board to the counter-clockwise
    pub fn rotate_left(&mut self) {
        self.board = Self::rotate_board_left(self.board.clone());
        self.update_display_board(-1);
    }

    /// Updates the display board based on a number of changes to the game board.
    /// Changes are in units of 90 degrees. Clockwise is positive, counter-clockwise is negative.
    ///
    /// Changes are summed into offset as a cache.
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

    use crate::logic::error::GameError;

    use super::GameBoard;

    #[test]
    pub fn verify_place_top() {
        let mut game_board = GameBoard::new(3);

        // top place
        game_board.place(0).unwrap();

        println!("{}", game_board.board);

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
        let mut game_board = GameBoard::new(4);

        [4, 4, 4, 3, 3, 3, 4].into_iter().for_each(|place| {
            println!("Placing {place}");
            game_board.place(place).unwrap();
            println!("Placed Board {}", game_board.board);
        });
    }

    #[test]
    pub fn verify_corner_case_2() {
        let mut game_board = GameBoard::new(4);

        [0, 0, 0, 15, 15, 15, 15].into_iter().for_each(|place| {
            println!("Placing {place}");
            game_board.place(place).unwrap();
            println!("Placed Board {}", game_board.board);
        });
    }

    #[test]
    pub fn verify_board_clearing() {
        let mut game_board = GameBoard::new(3).with_rows_clearing();

        game_board.place(0).unwrap();
        game_board.place(0).unwrap();

        game_board.place(11).unwrap();
        game_board.place(11).unwrap();

        assert_eq!(
            game_board.board(),
            &DMatrix::from_rows(&[
                RowDVector::from_vec(vec![0, 1, 1]),
                RowDVector::from_vec(vec![1, 0, 0]),
                RowDVector::from_vec(vec![1, 0, 0]),
            ])
        );

        game_board.place(0).unwrap();

        assert_eq!(
            game_board.board(),
            &DMatrix::from_rows(&[
                RowDVector::from_vec(vec![0, 0, 0]),
                RowDVector::from_vec(vec![0, 0, 0]),
                RowDVector::from_vec(vec![0, 0, 0]),
            ])
        );
    }

    #[test]
    pub fn verify_stacking() {
        let mut game_board = GameBoard::new(3);

        game_board.place(10).unwrap();
        game_board.place(6).unwrap();

        assert_eq!(
            game_board.board(),
            &DMatrix::from_rows(&[
                RowDVector::from_vec(vec![0, 0, 0]),
                RowDVector::from_vec(vec![0, 0, 1]),
                RowDVector::from_vec(vec![0, 0, 1]),
            ])
        );

        assert_eq!(game_board.place(6), Err(GameError::NoSpace));
    }

    #[test]
    pub fn verify_scoring() {
        let mut game_board = GameBoard::new(3).with_rows_clearing();

        game_board.place(0).unwrap();
        // With maximum drop mult (5) * 1 point (no other mults)
        assert_eq!(game_board.score.score(), 5);

        // Pass the time
        game_board.score.update(1000.0);

        game_board.place(0).unwrap();
        assert_eq!(game_board.score.score(), 6);

        // Pass the time
        game_board.score.update(1000.0);

        game_board.place(0).unwrap();

        // cleared row, mult increased by 1 to 2
        // so 1 + (clearing row of 3 * mult of 2) = delta of 7
        assert_eq!(game_board.score.score(), 13);
    }

    #[test]
    pub fn verify_scoring_2() {
        let mut game_board = GameBoard::new(3).with_rows_clearing();

        // Pass the time
        game_board.score.update(1000.0);

        game_board.place(0).unwrap();

        // Pass the time
        game_board.score.update(1000.0);

        game_board.place(0).unwrap();

        // Pass the time
        game_board.score.update(1000.0);

        game_board.place(11).unwrap();

        // Pass the time
        game_board.score.update(1000.0);

        game_board.place(11).unwrap();

        // Pass the time
        game_board.score.update(1000.0);

        game_board.place(11).unwrap();

        // 2 clears at once, mult is now 5 (1 + 4)
        // dim of 3 is 6 cleared, so 6 * 1 (time passed) * 5 added to a score of 5

        assert_eq!(game_board.score.score(), 35);
    }
}
