use std::{fs, io};

use crate::{
    messaging::error_log,
    patches::{apply_patches, Config},
};
use thiserror::Error;

#[derive(Debug, Error)]
#[error("{message}")]
pub struct InitError {
    message: String,
}
impl InitError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}
impl From<std::io::Error> for InitError {
    fn from(e: std::io::Error) -> Self {
        Self::new(e.to_string())
    }
}

pub unsafe fn init() -> Result<(), InitError> {
    let config_file = fs::read_to_string("quickstart.toml").or_else(|e| match e.kind() {
        // if the config file doesn't exist, treat it as a blank file
        io::ErrorKind::NotFound => {
            error_log!("Warn: quickstart.toml not found");
            Ok("".into())
        }
        _ => Err(e),
    })?;
    let config: Config = toml::from_str(&config_file).map_err(|e| {
        InitError::new("Formatting error in quickstart.toml: ".to_owned() + e.message())
    })?;

    apply_patches(&config);

    Ok(())
}
