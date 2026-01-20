use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct CustomError {
    message: String,
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
