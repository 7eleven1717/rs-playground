use std::error::Error;
use std::fmt::Display;

#[derive(Debug)]
pub enum VideoConfigError {
    UnsupportedPixelFormat,
}

impl Display for VideoConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VideoConfigError::UnsupportedPixelFormat => {
                write!(f, "Unsupported pixel format")
            }
        }
    }
}

impl Error for VideoConfigError {}
