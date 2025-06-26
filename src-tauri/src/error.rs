use std::io;

#[derive(Debug)]
pub enum AppError {
    Io(io::Error),
    Serialization(String),
    DirectoryNotFound,
    Rusqlite(rusqlite::Error),
    Keyring(keyring::Error),
    ClipboardHistory(String),
    Frecency(String),
    FileSearch(String),
}

impl From<io::Error> for AppError {
    fn from(error: io::Error) -> Self {
        AppError::Io(error)
    }
}

impl From<rusqlite::Error> for AppError {
    fn from(error: rusqlite::Error) -> Self {
        AppError::Rusqlite(error)
    }
}

impl From<keyring::Error> for AppError {
    fn from(error: keyring::Error) -> Self {
        AppError::Keyring(error)
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
            AppError::Rusqlite(err) => write!(f, "Database error: {}", err),
            AppError::Keyring(err) => write!(f, "Keychain error: {}", err),
            AppError::ClipboardHistory(msg) => write!(f, "Clipboard history error: {}", msg),
            AppError::Frecency(msg) => write!(f, "Frecency error: {}", msg),
            AppError::FileSearch(msg) => write!(f, "File search error: {}", msg),
        }
    }
}

impl std::error::Error for AppError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            AppError::Io(err) => Some(err),
            AppError::Rusqlite(err) => Some(err),
            AppError::Keyring(err) => Some(err),
            _ => None,
        }
    }
}
