//! Error handling.

use std::fmt::Display;
use std::io;
use std::result;

/// Library errors.
#[derive(Debug)]
pub enum Error {
    /// I/O error.
    IOError(io::Error),
    /// Not enough bytes to complete header
    HeaderTooShort(io::Error),
    /// LZMA error.
    LZMAError(String),
    /// XZ error.
    XZError(String),
}

/// Library result alias.
pub type Result<T> = result::Result<T, Error>;

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Error {
        Error::IOError(e)
    }
}

impl Display for Error {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::IOError(e) => write!(fmt, "io error: {}", e),
            Error::HeaderTooShort(e) => write!(fmt, "header too short: {}", e),
            Error::LZMAError(e) => write!(fmt, "lzma error: {}", e),
            Error::XZError(e) => write!(fmt, "xz error: {}", e),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::IOError(e) | Error::HeaderTooShort(e) => Some(e),
            Error::LZMAError(_) | Error::XZError(_) => None,
        }
    }
}

#[cfg(test)]
mod test {
    use super::Error;

    #[test]
    fn test_display() {
        assert_eq!(
            Error::IOError(std::io::Error::new(
                std::io::ErrorKind::Other,
                "this is an error"
            ))
            .to_string(),
            "io error: this is an error"
        );
        assert_eq!(
            Error::LZMAError("this is an error".to_string()).to_string(),
            "lzma error: this is an error"
        );
        assert_eq!(
            Error::XZError("this is an error".to_string()).to_string(),
            "xz error: this is an error"
        );
    }
}
