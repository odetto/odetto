use std::error::Error;
use std::fmt;
use std::iter::Peekable;

use crate::{
    lexer::{Token, Tokens, TokenIter}
};

#[derive(Debug)]
pub struct ParseError;

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Parsing error")
    }
}

impl Error for ParseError {}

#[derive(Debug)]
pub struct FieldDef {
    name: Token,
    field_type: Token,
}

#[derive(Debug)]
pub struct TypeDef {
    fields: Vec<FieldDef>
}

pub struct Parser<'a> {
    tokens: Peekable<TokenIter<'a>>
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a Tokens) -> Parser<'a> {
        Parser {
            tokens: tokens.into_iter().peekable()
        }
    }

    pub fn parse(&mut self) -> Result<Vec<TypeDef>, ParseError> {
        // @todo load config from yml
        let mut defs = Vec::new();

        loop {
            let next = self.next();

            match next {
                Some(n) => defs.push(n),
                None => return Err(ParseError)
            }
        }
    }

    fn next(&mut self) -> Option<TypeDef> {
        let mut t = if let Some(c) = self.tokens.peek() {
            *c
        } else {
            return None;
        };
        Some(TypeDef { fields: Vec::new() })
    }
}