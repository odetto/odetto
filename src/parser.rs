use std::iter::Peekable;

use crate::{
    _helpers::{ParseError, ParseResult, TokenInfo},
    lexer::{Token, Tokens, TokenIter, TokenType},
    ast::{Root, ModelTypeDef, FieldDef, FieldType, FieldTypeType},
};

pub struct Parser<'a> {
    tokens: Peekable<TokenIter<'a>>,
    model_identifiers: Vec<String>
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a Tokens) -> Parser<'a> {
        Parser {
            tokens: tokens.into_iter().peekable(),
            model_identifiers: Vec::new()
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

            if let Some(n) = self.tokens.peek() {
                if n.t == TokenType::EOF {
                    break;
                }
            }
        }

        Ok(root)
    }

    fn next_model(&mut self) -> ParseResult<Option<ModelTypeDef>> {
        let mut token = if let Some(t) = self.tokens.peek() {
            *t
        } else {
            return Ok(None);
        };

        if token.t != TokenType::FieldType {
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

        if self.model_identifiers.contains(&name) {
            return Err(ParseError::DuplicateModelIdentifierError(TokenInfo { loc: token.loc }))
        }


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

        self.model_identifiers.push(name.clone());
        Ok(Some(ModelTypeDef { name, fields, ..Default::default() }))
    }

    fn get_fields(&mut self) -> ParseResult<Vec<FieldDef>> {
        let mut fields = Vec::new();
        let mut field_names = Vec::new();

        let mut token = if let Some(t) = self.tokens.peek() {
            *t
        } else {
            return Err(ParseError::NoFieldsError);
        };

        while token.t != TokenType::CurlyR {
            if token.t != TokenType::Identifier {
                // identifier expected
                return Err(ParseError::ExpectedFieldIdentifierError(TokenInfo { loc: token.loc }))
            }

            let name = token.value.clone();

            if field_names.contains(&name) {
                return Err(ParseError::DuplicateFieldIdentifierError(TokenInfo { loc: token.loc }))
            }

            self.tokens.next();

            token = if let Some(t) = self.tokens.peek() {
                *t
            } else {
                // no type for field
                return Err(ParseError::MissingFieldTypeError(TokenInfo { loc: token.loc }));
            };

            if token.t != TokenType::Colon {
                // need colon to indicate type
                return Err(ParseError::MissingColonError(TokenInfo { loc: token.loc }));
            }

            self.tokens.next();

            // FIELD TYPE

            let (field_type, type_type) = self.parse_token_type()?;

            token = if let Some(t) = self.tokens.peek() {
                *t
            } else {
                // no curly brace to end it
                return Err(ParseError::MissingRightBracketError(TokenInfo { loc: token.loc }));
            };

            let mut field_required = false;

            if token.t == TokenType::OpExclamation {
                field_required = true;

                self.tokens.next();

                token = if let Some(t) = self.tokens.peek() {
                    *t
                } else {
                    // no curly brace to end it
                    return Err(ParseError::MissingRightBracketError(TokenInfo { loc: token.loc }));
                };
            }

            // CONSTRUCT FIELD

            field_names.push(name.clone());
            fields.push(FieldDef { name, field_type, type_type, required: field_required, ..Default::default() });

            // self.tokens.next();

            // token = if let Some(t) = self.tokens.peek() {
            //     *t
            // } else {
            //     // no curly brace to end it
            //     return Err(ParseError::MissingRightBracketError(TokenInfo { loc: token.loc }));
            // };
        }
       
        self.tokens.next();

        Ok(fields)
    }

    fn parse_token_type(&mut self) -> ParseResult<(FieldType, FieldTypeType)> {
        let field_type;
        let mut type_type = FieldTypeType::Basic;
        let mut token = if let Some(t) = self.tokens.peek() {
            *t
        } else {
            // no type for field
            return Err(ParseError::GenericFieldParsingError(None, String::from("Could not get next token when starting to parse field token type.")));
            // return None
        };

        // if start of array
        if token.t == TokenType::BracketL {
            type_type = FieldTypeType::Array;

            self.tokens.next();

            token = if let Some(t) = self.tokens.peek() {
                *t
            } else {
                return Err(ParseError::GenericFieldParsingError(Some(TokenInfo { loc: token.loc }), String::from("Could not get next token after opening array bracket.")));
            };
        }

        if token_is_type(&token) {
            field_type = FieldType::Scalar(token.value.clone());
        }
        else if token.t == TokenType::Identifier {
            field_type = FieldType::Identfier(token.value.clone());
        }
        else {
            return Err(ParseError::GenericFieldParsingError(Some(TokenInfo { loc: token.loc }), String::from("Field type could not be identified as scalar or identifier.")));
        }

        self.tokens.next();

        token = if let Some(t) = self.tokens.peek() {
            *t
        } else {
            return Err(ParseError::GenericFieldParsingError(Some(TokenInfo { loc: token.loc }), String::from("Could not get next token after getting field type.")));
        };

        if type_type == FieldTypeType::Array || type_type == FieldTypeType::RequiredArray {
            if token.t == TokenType::OpExclamation {
                type_type = FieldTypeType::RequiredArray;
    
                self.tokens.next();
    
                token = if let Some(t) = self.tokens.peek() {
                    *t
                } else {
                    return Err(ParseError::GenericFieldParsingError(Some(TokenInfo { loc: token.loc }), String::from("Could not get next token after required array field type found.")));
                };
            }

            if token.t == TokenType::BracketR {
                self.tokens.next();

                // token = if let Some(t) = self.tokens.peek() {
                //     *t
                // } else {
                //     return Err(ParseError::GenericFieldParsingError(Some(TokenInfo { loc: token.loc }), String::from("Could not get next token after required array field type found.")));
                // };
            } else {
                return Err(ParseError::GenericFieldParsingError(Some(TokenInfo { loc: token.loc }), String::from("Missing closing right bracket on field.")));
            }
        }

        Ok((field_type, type_type))
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