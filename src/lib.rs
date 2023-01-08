pub mod error;
pub mod tags;
pub mod traits;

pub use error::*;
pub use tags::*;
pub use traits::*;

use std::{ffi::OsStr, path::Path};

// TODO: possibly better way of doing this
fn string_from_os_str(
    os_str: Option<&OsStr>,
    process: &'static str,
    file_name: Option<&String>,
) -> Result<String, GetTagsError> {
    let generate_error = |message_format: &'static str| {
        GetTagsError::new(
            file_name.unwrap_or(&"test".to_string()).to_owned(),
            format!("{} {}", message_format, process),
        )
    };

    Ok(os_str
        .ok_or_else(|| generate_error("Could not retrieve"))?
        .to_str()
        .ok_or_else(|| generate_error("Could not parse"))?
        .to_string())
}

pub fn from(file_path: &impl AsRef<Path>) -> Result<Box<dyn Tags>, GetTagsError> {
    let file_path = file_path.as_ref();
    let file_name = string_from_os_str(Some(file_path.as_os_str()), "file path string", None)?;
    let file_contents = std::fs::read(file_path)?;

    let extension = string_from_os_str(file_path.extension(), "file extension", Some(&file_name))?;
    match extension.as_str() {
        ".flac" => Ok(Box::new(FlacTags {})),
        ".mp3" => Ok(Box::new(MP3Tags {})),
        ".m4a" => Ok(Box::new(MP4Tags {})),
        _ => Err(GetTagsError::new(
            file_name,
            format!("Unsupported file type: {}", extension),
        )),
    }
}
