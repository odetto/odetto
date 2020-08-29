use std::error::Error;
use std::fmt;
use std::iter::Peekable;

use crate::{
    lexer::{Token}
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

pub struct Parser {
    tokens: Peekable<Vec<Token>>
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser {
            tokens
        }
    }

    pub fn parse(&mut self) -> Result<Vec<TypeDef>, ParseError> {
        let mut defs = Vec::new();

        loop {
            let next = self.next();

            if next.is_none() {
                return Err(ParseError);
            } else {
                defs.push(next.unwrap());
            }
        }
    }

    fn next(&mut self) -> Option<TypeDef> {
        let mut t = if let Some(c) = self.tokens.peek() {
            *c
        } else {
            return Token::eof(self.index);
        };
        Some(TypeDef { fields: Vec::new() })
    }
}