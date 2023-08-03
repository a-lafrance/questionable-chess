#[derive(Debug, thiserror::Error)]
pub enum MoveError {
    #[error("invalid format")]
    InvalidFormat,

    #[error("piece cannot move along specified path")]
    InvalidPath,

    #[error("no piece at specified square")]
    PieceNotFound,

    #[error("piece at specified square has wrong color")]
    WrongPieceColor,

    #[error("piece at specified square is wrong kind")]
    WrongPieceKind,
}
