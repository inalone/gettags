use std::{error::Error, fmt};

#[derive(Debug, Clone)]
pub enum ErrorKind {
    InvalidFileExtension,
    IoError,
}

#[derive(Debug, Clone)]
pub struct GetTagsError {
    error_kind: ErrorKind,
    message: String,
}

impl GetTagsError {
    pub fn new(error_kind: ErrorKind, message: String) -> Self {
        GetTagsError {
            error_kind,
            message,
        }
    }
}

impl fmt::Display for GetTagsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Could not extract tags from file: {}", self.message)
    }
}

impl From<std::io::Error> for GetTagsError {
    fn from(e: std::io::Error) -> Self {
        GetTagsError {
            error_kind: ErrorKind::IoError,
            message: e.to_string(),
        }
    }
}

impl Error for GetTagsError {
    fn description(&self) -> &str {
        &self.message
    }
}
