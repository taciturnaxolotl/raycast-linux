use std::io;

#[derive(Debug)]
pub enum AppError {
    Io(io::Error),
    Serialization(String),
    DirectoryNotFound,
    CacheError(String),
}

impl From<io::Error> for AppError {
    fn from(error: io::Error) -> Self {
        AppError::Io(error)
    }
}

impl From<bincode::error::DecodeError> for AppError {
    fn from(error: bincode::error::DecodeError) -> Self {
        AppError::Serialization(format!("Decode error: {}", error))
    }
}

impl From<bincode::error::EncodeError> for AppError {
    fn from(error: bincode::error::EncodeError) -> Self {
        AppError::Serialization(format!("Encode error: {}", error))
    }
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppError::Io(err) => write!(f, "IO error: {}", err),
            AppError::Serialization(msg) => write!(f, "Serialization error: {}", msg),
            AppError::DirectoryNotFound => write!(f, "Directory not found"),
            AppError::CacheError(msg) => write!(f, "Cache error: {}", msg),
        }
    }
}

impl std::error::Error for AppError {}
