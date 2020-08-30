use std::iter::{Peekable};
use std::str::{Chars};
use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub enum TokenType {
    Unknown,
    Literal,

    KeyType,

    TInt,
    TString,

    OpPlus,
    OpMinus,
    OpStar,
    OpForwSlash,
    OpExclamation,
    OpArrow,

    ParenL,
    ParenR,
    CurlyL,
    CurlyR,
    BracketL,
    BracketR,

    Colon,
    DocString,

    EOF,
}

#[derive(Clone, Debug)]
pub struct Token {
    pub t: TokenType,
    pub value: String,
    pub loc: (usize, usize) // @todo look into getting col and row for location
}

impl Token {
    fn eof(index: usize) -> Token {
        Token {
            t: TokenType::EOF,
            value: "".to_string(),
            loc: (index, index)
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        
        write!(f, "{:?} : ({}, {}), value = {}\n", self.t, self.loc.0, self.loc.1, self.value)
    }
}

pub struct TokenIter<'a> {
    iter: std::slice::Iter<'a, Token>,
}

#[derive(Clone, Debug)]
pub struct Tokens {
    tokens: Vec<Token>
}

impl fmt::Display for Tokens {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let vec = &self.tokens;

        write!(f, "[\n")?;
        for (_, v) in vec.iter().enumerate() {
            let mut value = String::new();
            if v.value.len() > 0 {
                value = format!(", value = '{}'", v.value);
            }
            write!(f, "\t{:?}: ({}, {}){}\n", v.t, v.loc.0, v.loc.1, value)?;
        }
        write!(f, "]\n")
        
    }
}

impl<'a> IntoIterator for &'a Tokens {
    type Item = &'a Token;
    type IntoIter = TokenIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        TokenIter {
            iter: self.tokens.iter()
        }
    }
}

impl<'a> Iterator for TokenIter<'a> {
    type Item = &'a Token;

    // just return the str reference
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

pub struct Lexer<'a> {
    orginal: &'a str,
    chars: Peekable<Chars<'a>>,
    index: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(src: &str) -> Lexer {
        Lexer {
            orginal: src,
            chars: src.chars().peekable(),
            index: 0,
        }
    }

    pub fn run(&mut self) -> Tokens {
        let mut tokens = Vec::new();

        loop {
            let next = self.next();

            if let TokenType::EOF = next.t {
                tokens.push(next.clone());
                break;
            } else {
                tokens.push(next.clone());
            }
        }

        Tokens { tokens }
    }

    fn next(&mut self) -> Token {
        let mut c = if let Some(c) = self.peek() {
            *c
        } else {
            return Token::eof(self.index);
        };

        while WHITESPACE.contains(&c) {
            let next_char = self.advance();
            if next_char == None || self.peek() == None {
                return Token::eof(self.index);
            }

            c = *self.peek().unwrap();
        }

        if c == '"' {
            if let Some(co) = self.next_doc_string() {
                return co;
            }
            // look for string literal here later?
            let next_char = self.advance();
            if next_char == None || self.peek() == None {
                return Token::eof(self.index);
            }

            c = *self.peek().unwrap();
            while WHITESPACE.contains(&c) {
                let next_char = self.advance();
                if next_char == None || self.peek() == None {
                    return Token::eof(self.index);
                }

                c = *self.peek().unwrap();
            }
        }

        if c == '#' {
            while !NEW_LINE.contains(&c)  {
                let next_char = self.advance();
                if next_char == None || self.peek() == None {
                    return Token::eof(self.index);
                }

                c = *self.peek().unwrap();
            }

            while WHITESPACE.contains(&c) {
                let next_char = self.advance();
                if next_char == None || self.peek() == None {
                    return Token::eof(self.index);
                }

                c = *self.peek().unwrap();
            }
        }

        if is_special_identifier(Some(&c)) {
            if let Some(si) = self.next_special_identifier() {
                return si;
            }
        }

        if is_valid_identifier(Some(&c)) {
            return self.next_identifier();
        }

        let loc = (self.index, self.index + 1);
        let token = match c {
            '+' => Token { t: TokenType::OpPlus, value: String::new(), loc },
            '-' => Token { t: TokenType::OpMinus, value: String::new(), loc },
            '*' => Token { t: TokenType::OpStar, value: String::new(), loc },
            '/' => Token { t: TokenType::OpForwSlash, value: String::new(), loc },
            '!' => Token { t: TokenType::OpExclamation, value: String::new(), loc },
            '(' => Token { t: TokenType::ParenL, value: String::new(), loc },
            ')' => Token { t: TokenType::ParenR, value: String::new(), loc },
            '{' => Token { t: TokenType::CurlyL, value: String::new(), loc },
            '}' => Token { t: TokenType::CurlyR, value: String::new(), loc },
            '[' => Token { t: TokenType::BracketL, value: String::new(), loc },
            ']' => Token { t: TokenType::BracketR, value: String::new(), loc },
            ':' => Token { t: TokenType::Colon, value: String::new(), loc },
            _ => Token { t: TokenType::Unknown, value: c.to_string(), loc }
        };

        self.advance();

        token
    }

    fn peek(&mut self) -> Option<&char> {
        self.chars.peek()
    }

    fn advance(&mut self) -> Option<char> {
        self.index += 1;
        self.chars.next()
    }

    fn back(&mut self, index: usize) {
        self.index = index - 1;
        self.chars = self.orginal.chars().peekable();
        self.chars.nth(index - 1);
    }

    fn next_identifier(&mut self) -> Token {
        let start = self.index;
        let mut end = start;
        let letter_regex = regex::Regex::new(r"[A-z'_]").unwrap();
        let mut value = String::new();

        while is_valid_identifier(self.peek()) {
            let c = *self.peek().unwrap();

            if end - start > 0 && !letter_regex.is_match(&c.to_string()) {
                break;
            }

            end += 1;
            value.push(c);
            self.advance();
        }

        let t = match value.as_ref() {
            "type" => TokenType::KeyType,
            "Int" => TokenType::TInt,
            "String" => TokenType::TString,
            _ => TokenType::Literal,
        };

        Token {
            t: t,
            value: value,
            loc: (start, end)
        }
    }

    fn next_special_identifier(&mut self) -> Option<Token> {
        let start = self.index;
        let mut end = start;
        let special_regex = regex::Regex::new(r"[->]").unwrap();
        let mut value = String::new();

        while is_special_identifier(self.peek()) {
            let c = *self.peek().unwrap();

            if end - start > 0 && !special_regex.is_match(&c.to_string()) {
                break;
            }

            end += 1;
            value.push(c);
            self.advance();
        }

        match value.as_ref() {
            "->" => Some(Token { t: TokenType::OpArrow, value, loc: (start, end) }),
            _ => {
                self.back(start);
                None
            }
        }
    }

    fn next_doc_string(&mut self) -> Option<Token> {
        let start = self.index;
        let mut end = start;
        let mut value = String::new();

        while is_quote(self.peek()) && value.len() < 3 {
            let c = *self.peek().unwrap();

            if end - start > 0 && c != '"' {
                break;
            }

            end += 1;
            value.push(c);
            self.advance();
        }

        if value != "\"\"\"" {
            self.back(start);
            return None;
        }

        // while not new triple quote save chars into string
        let mut doc_string = String::new();
        value = String::new();
        
        let start_doc = self.index;

        while value != "\"\"\"" {
            let c = if let Some(c) = self.peek() {
                *c
            } else {
                return None;
            };

            let index = self.index;
            if c == '"' {
                // try to find end of doc string
                value.push(c);
                self.advance();
                let c1 = if let Some(c) = self.peek() {
                    *c
                } else {
                    return None;
                };
                if c1 == '"' {
                    value.push(c1);
                    self.advance();
                    let c2 = if let Some(c) = self.peek() {
                        *c
                    } else {
                        return None;
                    };
                    if c2 == '"' {
                        value.push(c2);
                        self.advance();
                        break;
                    } else {
                        value = String::new();
                        self.back(index + 1); // go back to after the first quote
                    }
                } else {
                    value = String::new();
                    self.back(index + 1); // go back to after the first quote
                }
            }

            end += 1;
            doc_string.push(c);
            self.advance();
        }

        let trimmed = doc_string.trim().to_string();

        if trimmed.len() == 0 {
            return None;
        }

        Some(Token { t: TokenType::DocString, value: trimmed, loc: (start_doc, end)})
    }
}

fn is_valid_identifier(c: Option<&char>) -> bool {
    if let Some(c) = c {
        regex::Regex::new(r#"[^\s\n\r0-9\+-/\*\^!#\(\)\{\}=\.,:;|"'\[\]]"#)
            .unwrap()
            .is_match(&c.to_string())
    } else {
        false
    }
}

fn is_special_identifier(c: Option<&char>) -> bool {
    if let Some(c) = c {
        regex::Regex::new(r"[->]")
            .unwrap()
            .is_match(&c.to_string())
    } else {
        false
    }
}

fn is_quote(c: Option<&char>) -> bool {
    if let Some(c) = c {
        return c == &'"';
    } else {
        false
    }
}

const WHITESPACE: [char; 4] = [' ', '\n', '\r', '\t'];
const NEW_LINE: [char; 2] = ['\n', '\r'];

#[cfg(test)]
mod tests {
    use super::*;

    fn match_tokens(tokens: Tokens, expected: Vec<TokenType>) {
        let mut expected_iter = expected.iter();

        for token in tokens.tokens {
            assert_eq!(token.t, *expected_iter.next().unwrap());
        }
    }

    #[test]
    fn token_types() {
        let mut l = Lexer::new("+-*/!->()[]{}:");
        let tokens = l.run();
        let expected = vec![
            TokenType::OpPlus,
            TokenType::OpMinus,
            TokenType::OpStar,
            TokenType::OpForwSlash,
            TokenType::OpExclamation,
            TokenType::OpArrow,
            TokenType::ParenL,
            TokenType::ParenR,
            TokenType::BracketL,
            TokenType::BracketR,
            TokenType::CurlyL,
            TokenType::CurlyR,
            TokenType::Colon,
            TokenType::EOF,
        ];

        match_tokens(tokens, expected);
    }

    #[test]
    fn skip_comments() {
        let mut l = Lexer::new("# hello tests! I am here -> () #\n testing token # inline comment to eof");
        let tokens = l.run();
        let expected = vec![
            TokenType::Literal,
            TokenType::Literal,
            TokenType::EOF,
        ];

        match_tokens(tokens, expected);
    }

    #[test]
    fn doc_strings() {
        let mut l = Lexer::new(r#"type Movie """\n  \t  it's "" so good to be here\n""" "#);
        let tokens = l.run();
        let expected = vec![
            TokenType::KeyType,
            TokenType::Literal,
            TokenType::DocString,
            TokenType::EOF,
        ];

        match_tokens(tokens, expected);
    }
    #[test]
    fn empty_doc_string() {
        let mut l = Lexer::new(r#"blah blah """""" 
             empty "#);
        let tokens = l.run();
        let expected = vec![
            TokenType::Literal,
            TokenType::Literal,
            TokenType::Literal,
            TokenType::EOF,
        ];

        println!("{}", tokens);

        match_tokens(tokens, expected);
    }
}
