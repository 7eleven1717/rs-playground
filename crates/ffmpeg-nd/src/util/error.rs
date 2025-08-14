use core::ffi::c_int;
use core::fmt;
use std::backtrace::Backtrace;

#[derive(Debug)]
pub enum Error {
    /// For AVERROR(e) wrapping POSIX error codes, e.g. AVERROR(EAGAIN).
    Other { errno: c_int, backtrace: Backtrace },

    /// For non-FFmpeg string errors.
    Message(String),
}

impl Error {
    pub fn is_eagain(&self) -> bool {
        matches!(self, Error::Other { errno: e, backtrace: _ } if *e == crate::util::_util::AVERROR_EAGAIN)
    }

    pub fn is_eof(&self) -> bool {
        matches!(self, Error::Other { errno: e, backtrace: _ } if *e == crate::util::_util::AVERROR_EOF)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Other { errno, backtrace } => write!(
                f,
                "FFmpeg error with code: {}\nBacktrace: {:?}",
                errno, backtrace
            ),
            Error::Message(msg) => write!(f, "Error: {}", msg),
        }
    }
}

impl std::error::Error for Error {}

impl From<c_int> for Error {
    fn from(value: c_int) -> Error {
        match value {
            e => Error::Other {
                errno: e,
                backtrace: Backtrace::capture(),
            },
        }
    }
}

impl From<&str> for Error {
    fn from(msg: &str) -> Error {
        Error::Message(msg.to_owned())
    }
}

impl From<String> for Error {
    fn from(msg: String) -> Error {
        Error::Message(msg)
    }
}
