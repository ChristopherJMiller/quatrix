use thiserror::Error;

#[derive(Debug, Error)]
pub enum GameError {
    #[error(
        "Invalid Block Placement at location {0} (starting from the top left and going clockwise)"
    )]
    InvalidPlacementLocation(usize),
    #[error("Slice was too full to place")]
    SliceFull,
}
