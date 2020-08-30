use std::iter::Peekable;

use crate::{
    _helpers::{ParseError, ParseResult, TokenInfo},
    lexer::{Token, Tokens, TokenIter, TokenType},
    ast::{Root, ModelTypeDef},
};

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

    pub fn parse(&mut self) -> ParseResult<Root> {
        // @todo load config from yml
        let mut root = Root::new();

        loop {
            let next = self.next_model()?;

            match next {
                Some(n) => root.types.push(n),
                None => break,
            }
            break;
        }

        Ok(root)
    }

    fn next_model(&mut self) -> ParseResult<Option<ModelTypeDef>> {
        let mut token = if let Some(t) = self.tokens.peek() {
            *t
        } else {
            return Ok(None);
        };

        if token.t != TokenType::KeyType {
            return Err(ParseError::GenericError(TokenInfo { loc: token.loc }))
        }

        self.tokens.next();

        token = if let Some(t) = self.tokens.peek() {
            *t
        } else {
            // expected literal but there was nothing
            return Err(ParseError::GenericError(TokenInfo { loc: token.loc }))
        };

        if token.t != TokenType::Literal {
            // expected literal but got <tokentype>
            return Err(ParseError::GenericError(TokenInfo { loc: token.loc }))
        }

        let name = token.value.clone();

        self.tokens.next();

        token = if let Some(t) = self.tokens.peek() {
            *t
        } else {
            // expected left curly bracket but got nothing
            return Err(ParseError::GenericError(TokenInfo { loc: token.loc }))
        };

        if token.t != TokenType::CurlyL {
            // expected left curly bracket but got <tokentype>
            return Err(ParseError::GenericError(TokenInfo { loc: token.loc }))
        }

        // find model pattern 

        // let fields = self.get_fields();

        Ok(Some(ModelTypeDef { name, fields: Vec::new() }))
    }

    // fn peek(&mut self) -> Option<&char> {
    //     self.chars.peek()
    // }

    // fn advance(&mut self) -> Option<char> {
    //     self.index += 1;
    //     self.chars.next()
    // }

    // fn back(&mut self, index: usize) {
    //     self.index = index - 1;
    //     self.chars = self.orginal.chars().peekable();
    //     self.chars.nth(index - 1);
    // }
}