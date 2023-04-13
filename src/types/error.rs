use std::fmt::Display;

#[derive(Debug)]
pub enum Error {
    StringParseError(std::string::FromUtf8Error),
    IoError(std::io::Error),
    AppNotFoundError,
    ResponseParseError,
}

impl Display for Error {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            _ => std::fmt::Result::Err(std::fmt::Error {}),
        }
    }
}

impl std::error::Error for Error {}

impl Into<Vec<u8>> for Error {
    fn into(self) -> Vec<u8> {
        let mut bytes: Vec<u8> = vec![];
        bytes.extend_from_slice(match &self {
            Self::StringParseError(_) => &[0],
            Self::IoError(_) => &[1],
            Self::AppNotFoundError => &[2],
            Self::ResponseParseError => &[3],
        });
        bytes
    }
}
