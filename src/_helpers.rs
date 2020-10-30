use std::error::Error;
use std::fmt;

pub type ParseResult<T> = std::result::Result<T, ParseError>;

#[derive(Clone, Debug)]
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
    GenericFieldParsingError(Option<TokenInfo>, String),
    MissingRightBracketError(TokenInfo),
    ExpectedFieldIdentifierError(TokenInfo),
    DuplicateFieldIdentifierError(TokenInfo),
    MissingModelTypeError(TokenInfo),
    GenericError(TokenInfo)
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &*self {
            ParseError::UnknownError => write!(f, "Uknown parsing error\n"),
            ParseError::NoFieldsError => write!(f, "No fields present on type. \n"),
            ParseError::DuplicateModelIdentifierError(ref info) => write!(f, "Duplicate model identifier: ({}, {})", info.loc.0, info.loc.1),
            ParseError::MissingColonError(ref info) => write!(f, "Missing colon to indicate type on field: ({}, {})", info.loc.0, info.loc.1),
            ParseError::MissingFieldTypeError(ref info) => write!(f, "Missing type on field: ({}, {})", info.loc.0, info.loc.1),
            ParseError::GenericFieldTypeError(ref info) => write!(f, "Something is wrong with the type on field: ({}, {})", info.loc.0, info.loc.1),
            ParseError::GenericFieldParsingError(ref info, message) => {
                let optional_location = if let Some(i) = info {
                    format!(": ({}, {})", i.loc.0, i.loc.1)
                } else {
                    String::from(".")
                };
                write!(f, "Something is wrong with the type on field{}\nError message: {}", optional_location, message)
            },
            ParseError::MissingRightBracketError(ref info) => write!(f, "Missing '}}' to close the type definition: ({}, {})", info.loc.0, info.loc.1),
            ParseError::ExpectedFieldIdentifierError(ref info) => write!(f, "Expected a field identifier: ({}, {})", info.loc.0, info.loc.1),
            ParseError::DuplicateFieldIdentifierError(ref info) => write!(f, "Duplicate field identifier: ({}, {})", info.loc.0, info.loc.1),
            ParseError::MissingModelTypeError(ref info) => write!(f, "Missing model type: ({}, {})", info.loc.0, info.loc.1),
            ParseError::GenericError(ref info) => write!(f, "Generic parsing error: ({}, {})", info.loc.0, info.loc.1)
        }
    }
}

impl Error for ParseError {}