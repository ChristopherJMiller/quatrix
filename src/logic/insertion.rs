use nalgebra::DMatrix;

use super::error::GameError;

#[derive(Debug, PartialEq)]
pub enum InsertionDirection {
    FromTop,
    FromRight,
    FromBottom,
    FromLeft,
}

impl InsertionDirection {
    pub fn get_side_index(&self, board: &DMatrix<u8>, slot: usize) -> usize {
        let passed_slots = match self {
            InsertionDirection::FromTop => 0,
            InsertionDirection::FromRight => board.ncols(),
            InsertionDirection::FromBottom => board.ncols() + board.nrows(),
            InsertionDirection::FromLeft => board.ncols() + board.nrows() + board.ncols(),
        };

        let result = slot - passed_slots;

        match self {
            InsertionDirection::FromBottom => board.nrows().saturating_sub(result + 1),
            InsertionDirection::FromLeft => board.ncols().saturating_sub(result + 1),
            _ => result,
        }
    }

    pub fn for_board_insertion(
        board: &DMatrix<u8>,
        slot: usize,
    ) -> Result<InsertionDirection, GameError> {
        let width = board.ncols();
        let height = board.nrows();

        if width > slot {
            return Ok(InsertionDirection::FromTop);
        }
        let top_insertion = slot.saturating_sub(width);

        if height > top_insertion {
            return Ok(InsertionDirection::FromRight);
        }
        let right_insertion = top_insertion.saturating_sub(height);

        if width > right_insertion {
            return Ok(InsertionDirection::FromBottom);
        }
        let bottom_insertion = right_insertion.saturating_sub(width);

        if height > bottom_insertion {
            return Ok(InsertionDirection::FromLeft);
        }

        Err(GameError::InvalidPlacementLocation(slot))
    }

    pub fn place(&self, slice: &mut [u8]) -> Result<(), GameError> {
        let first_one_found = match self {
            InsertionDirection::FromTop | InsertionDirection::FromLeft => {
                slice.iter().position(|&x| x == 1)
            }
            InsertionDirection::FromBottom | InsertionDirection::FromRight => slice
                .iter()
                .rev()
                .position(|&x| x == 1)
                .map(|x| slice.len() - x - 1),
        };

        if let Some(one_found) = first_one_found {
            for i in one_found..slice.len() {
                let space_available = slice.get(i).map(|&x| x == 0).is_some_and(|x| x);

                if space_available {
                    slice[i] = 1;
                    return Ok(());
                }
            }

            // Place before
            if one_found > 0 {
                if slice.get(one_found - 1).map(|&x| x == 0).is_some_and(|x| x) {
                    slice[one_found - 1] = 1;
                    return Ok(());
                }
            }

            // Otherwise, error as it's full
            return Err(GameError::SliceFull);
        } else {
            let default_placement = match self {
                InsertionDirection::FromTop | InsertionDirection::FromLeft => slice.len() - 1,
                InsertionDirection::FromBottom | InsertionDirection::FromRight => 0,
            };

            slice[default_placement] = 1;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::InsertionDirection;
    use nalgebra::DMatrix;
    use std::ops::Range;

    fn matrix_iter(size_range: Range<usize>) -> impl Iterator<Item = Box<DMatrix<u8>>> {
        size_range
            .into_iter()
            .map(|n| Box::new(DMatrix::zeros(n, n)))
    }

    #[test]
    pub fn verify_top_insertion() {
        matrix_iter(2..10).for_each(|matrix| {
            (0..matrix.ncols()).into_iter().for_each(|slot| {
                let direction = InsertionDirection::for_board_insertion(&matrix, slot).unwrap();
                assert_eq!(
                    direction,
                    InsertionDirection::FromTop,
                    "Failure on slot {slot}"
                )
            });
        });
    }

    #[test]
    pub fn verify_right_insertion() {
        matrix_iter(2..10).for_each(|matrix| {
            (matrix.ncols()..(matrix.ncols() + matrix.nrows()))
                .into_iter()
                .for_each(|slot| {
                    let direction = InsertionDirection::for_board_insertion(&matrix, slot).unwrap();
                    assert_eq!(
                        direction,
                        InsertionDirection::FromRight,
                        "Failure on slot {slot}"
                    )
                });
        });
    }

    #[test]
    pub fn verify_bottom_insertion() {
        matrix_iter(2..10).for_each(|matrix| {
            ((matrix.ncols() + matrix.nrows())..(matrix.ncols() + matrix.nrows() + matrix.ncols()))
                .into_iter()
                .for_each(|slot| {
                    let direction = InsertionDirection::for_board_insertion(&matrix, slot).unwrap();
                    assert_eq!(
                        direction,
                        InsertionDirection::FromBottom,
                        "Failure on slot {slot}"
                    )
                });
        });
    }

    #[test]
    pub fn verify_left_insertion() {
        matrix_iter(2..10).for_each(|matrix| {
            let lower = matrix.ncols() + matrix.nrows() + matrix.ncols();
            let upper = matrix.ncols() + matrix.nrows() + matrix.ncols() + matrix.nrows();

            (lower..upper).into_iter().for_each(|slot| {
                let direction = InsertionDirection::for_board_insertion(&matrix, slot).unwrap();
                assert_eq!(
                    direction,
                    InsertionDirection::FromLeft,
                    "Failure on slot {slot}"
                )
            });
        });
    }

    #[test]
    pub fn verify_failures() {
        matrix_iter(2..10).for_each(|matrix| {
            ((matrix.ncols() + matrix.nrows() + matrix.ncols() + matrix.nrows())
                ..(matrix.ncols() + matrix.nrows() + matrix.ncols() + matrix.nrows()) * 2)
                .into_iter()
                .for_each(|slot| {
                    let direction = InsertionDirection::for_board_insertion(&matrix, slot);
                    assert!(direction.is_err());
                });
        });
    }

    #[test]
    pub fn verify_get_side_index() {
        let board: DMatrix<u8> = DMatrix::zeros(3, 3);

        assert_eq!(InsertionDirection::FromTop.get_side_index(&board, 0), 0);
        assert_eq!(InsertionDirection::FromTop.get_side_index(&board, 1), 1);
        assert_eq!(InsertionDirection::FromTop.get_side_index(&board, 2), 2);

        assert_eq!(InsertionDirection::FromRight.get_side_index(&board, 3), 0);
        assert_eq!(InsertionDirection::FromRight.get_side_index(&board, 4), 1);
        assert_eq!(InsertionDirection::FromRight.get_side_index(&board, 5), 2);

        assert_eq!(InsertionDirection::FromBottom.get_side_index(&board, 6), 2);
        assert_eq!(InsertionDirection::FromBottom.get_side_index(&board, 7), 1);
        assert_eq!(InsertionDirection::FromBottom.get_side_index(&board, 8), 0);

        assert_eq!(InsertionDirection::FromLeft.get_side_index(&board, 9), 2);
        assert_eq!(InsertionDirection::FromLeft.get_side_index(&board, 10), 1);
        assert_eq!(InsertionDirection::FromLeft.get_side_index(&board, 11), 0);
    }
}
