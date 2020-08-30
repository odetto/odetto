use std::error::Error;
use std::fmt;

pub type ParseResult<T> = std::result::Result<T, ParseError>;

#[derive(Debug)]
pub struct TokenInfo {
    pub loc: (usize, usize)
}

#[derive(Debug)]
pub enum ParseError {
    UnknownError,

    GenericError(TokenInfo)
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            ParseError::UnknownError => write!(f, "Uknown parsing error\n"),
            ParseError::GenericError(ref info) => write!(f, "Generic parsing error at ({}, {})", info.loc.0, info.loc.1)
        }
    }
}

impl Error for ParseError {}