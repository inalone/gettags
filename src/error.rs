use std::{error::Error, fmt};

// TODO: error enum
#[derive(Debug, Clone)]
pub struct GetTagsError {
    file_name: String,
    message: String,
}

impl GetTagsError {
    pub fn new(file_name: String, message: String) -> GetTagsError {
        GetTagsError { file_name, message }
    }
}

impl fmt::Display for GetTagsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "There was an error extracting tags from {}: {}",
            self.file_name, self.message
        )
    }
}

impl From<std::io::Error> for GetTagsError {
    fn from(value: std::io::Error) -> Self {
        todo!()
    }
}

impl Error for GetTagsError {}
