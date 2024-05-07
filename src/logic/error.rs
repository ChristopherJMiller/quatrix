use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum GameError {
    #[error(
        "Invalid Block Placement at location {0} (starting from the top left and going clockwise)"
    )]
    InvalidPlacementLocation(usize),
    #[error("No space from this angle to place")]
    NoSpace,
}
