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
}
