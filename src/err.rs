#[derive(Debug, thiserror::Error)]
pub enum MoveError {
    #[error("invalid format")]
    InvalidFormat,
}
