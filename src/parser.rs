use std::iter::Peekable;

use crate::{
    _helpers::{ParseError, ParseResult, TokenInfo},
    lexer::{Token, Tokens, TokenIter, TokenType},
    ast::{Root, ModelTypeDef, FieldDef},
};

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
            // expected Identifier but there was nothing
            return Err(ParseError::GenericError(TokenInfo { loc: token.loc }))
        };

        if token.t != TokenType::Identifier {
            // expected Identifier but got <tokentype>
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

        self.tokens.next();

        let fields = self.get_fields()?;

        Ok(Some(ModelTypeDef { name, fields }))
    }

    fn get_fields(&mut self) -> ParseResult<Vec<FieldDef>> {
        let mut fields = Vec::new();

        let mut token = if let Some(t) = self.tokens.peek() {
            *t
        } else {
            return Err(ParseError::NoFieldsError);
        };

        while token.t != TokenType::CurlyR {
            if token.t != TokenType::Identifier {
                return Err(ParseError::GenericError(TokenInfo { loc: token.loc }))
            }

            let name = token.value.clone();

            self.tokens.next();

            token = if let Some(t) = self.tokens.peek() {
                *t
            } else {
                // no type for field
                return Err(ParseError::GenericError(TokenInfo { loc: token.loc }));
            };

            if token.t != TokenType::Colon {
                // need colon to indicate type
                return Err(ParseError::GenericError(TokenInfo { loc: token.loc }));
            }

            self.tokens.next();

            token = if let Some(t) = self.tokens.peek() {
                *t
            } else {
                // no type for field
                return Err(ParseError::GenericError(TokenInfo { loc: token.loc }));
            };

            if !token_is_type(&token) {
                // type does not exist in context
                return Err(ParseError::GenericError(TokenInfo { loc: token.loc }));
            }

            let field_type = token.value.clone();

            fields.push(FieldDef { name, field_type });

            self.tokens.next();

            token = if let Some(t) = self.tokens.peek() {
                *t
            } else {
                // no curly brace to end it
                return Err(ParseError::GenericError(TokenInfo { loc: token.loc }));
            };
        }
       

        Ok(fields)
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

fn token_is_type(token: &Token) -> bool {
    token.t == TokenType::TInt || token.t == TokenType::TString
}