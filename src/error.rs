use thiserror::Error;

#[derive(Error, Debug)]
pub enum EcError {
    #[error("Authentication failed: missing cookie")]
    MissingCookie,

    #[error("Invalid day: {0} (must be 1-20)")]
    InvalidDay(i32),

    #[error("Invalid part: {0} (must be 1-3)")]
    InvalidPart(i32),

    #[error("Invalid year: {0}")]
    InvalidYear(i32),

    #[error("Decryption failed: {0}")]
    DecryptionError(String),

    #[error("API request failed: {0}")]
    ApiError(#[from] reqwest::Error),

    #[error("Answer already submitted")]
    AlreadySubmitted,

    #[error("Quest not available yet: {year}/{day} part {part}")]
    QuestNotAvailable { year: i32, day: i32, part: i32 },

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("JSON parsing error: {0}")]
    JsonError(#[from] serde_json::Error),

    #[error("Hex decoding error: {0}")]
    HexError(#[from] hex::FromHexError),

    #[error("HTTP error: {status} - {message}")]
    HttpError { status: u16, message: String },
}

pub type Result<T> = std::result::Result<T, EcError>;
