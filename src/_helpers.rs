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

    NoFieldsError,
    DuplicateModelIdentifierError(TokenInfo),
    MissingColonError(TokenInfo),
    MissingFieldTypeError(TokenInfo),
    GenericFieldTypeError(TokenInfo),
    MissingRightBracketError(TokenInfo),
    ExpectedFieldIdentifierError(TokenInfo),
    DuplicateFieldIdentifierError(TokenInfo),
    GenericError(TokenInfo)
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            ParseError::UnknownError => write!(f, "Uknown parsing error\n"),
            ParseError::NoFieldsError => write!(f, "No fields present on type. \n"),
            ParseError::DuplicateModelIdentifierError(ref info) => write!(f, "Duplicate model identifier: ({}, {})", info.loc.0, info.loc.1),
            ParseError::MissingColonError(ref info) => write!(f, "Missing colon to indicate type on field: ({}, {})", info.loc.0, info.loc.1),
            ParseError::MissingFieldTypeError(ref info) => write!(f, "Missing type on field: ({}, {})", info.loc.0, info.loc.1),
            ParseError::GenericFieldTypeError(ref info) => write!(f, "Something is wrong with the type on field: ({}, {})", info.loc.0, info.loc.1),
            ParseError::MissingRightBracketError(ref info) => write!(f, "Missing '}}' to close the type definition: ({}, {})", info.loc.0, info.loc.1),
            ParseError::ExpectedFieldIdentifierError(ref info) => write!(f, "Expected a field identifier: ({}, {})", info.loc.0, info.loc.1),
            ParseError::DuplicateFieldIdentifierError(ref info) => write!(f, "Duplicate field identifier: ({}, {})", info.loc.0, info.loc.1),
            ParseError::GenericError(ref info) => write!(f, "Generic parsing error: ({}, {})", info.loc.0, info.loc.1)
        }
    }
}

impl Error for ParseError {}