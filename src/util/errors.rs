use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct CustomError {
    pub message: String,
}

impl CustomError {
    pub fn new(messgae: &str) -> Self {
        Self {
            message: messgae.to_string(),
        }
    }
}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for CustomError {}

/// CredMan Error
pub enum CMError {
    RusqlilteError(rusqlite::Error),
    IOError(std::io::Error),
    Custom(CustomError),
}

impl fmt::Display for CMError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::RusqlilteError(err) => write!(f, "Database Error: {}", err),
            Self::IOError(err) => write!(f, "IO error: {}", err),
            Self::Custom(err) => write!(f, "Error: {}", err),
        }
    }
}

impl From<rusqlite::Error> for CMError {
    fn from(value: rusqlite::Error) -> Self {
        CMError::RusqlilteError(value)
    }
}

impl From<std::io::Error> for CMError {
    fn from(value: std::io::Error) -> Self {
        CMError::IOError(value)
    }
}

impl From<CustomError> for CMError {
    fn from(value: CustomError) -> Self {
        CMError::Custom(value)
    }
}
