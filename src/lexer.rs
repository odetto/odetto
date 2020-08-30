use std::iter::{Peekable};
use std::str::{Chars};
use std::fmt;

#[derive(Clone, Debug)]
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

    EOF,
}

#[derive(Clone, Debug)]
pub struct Token {
    t: TokenType,
    value: String,
    loc: (usize, usize)
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

        // Iterate over `v` in `vec` while enumerating the iteration
        // count in `count`.
        for (_, v) in vec.iter().enumerate() {
            let mut value = String::new();
            if v.value.len() > 0 {
                value = format!(", value = '{}'", v.value);
            }
            write!(f, "\t{:?}: ({}, {}){}\n", v.t, v.loc.0, v.loc.1, value)?;
        }

        // Close the opened bracket and return a fmt::Result value.
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

        println!("should not be here: {}", c);

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
        self.index = index;
        self.chars = self.orginal.chars().peekable();
        self.chars.nth(index);
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
                self.back(start - 1);
                None
            }
        }
    }
}

fn is_valid_identifier(c: Option<&char>) -> bool {
    if let Some(c) = c {
        regex::Regex::new(r"[^\s\n\r0-9\+-/\*\^!#\(\)\{\}=\.,:;|\[\]]")
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

const WHITESPACE: [char; 4] = [' ', '\n', '\r', '\t'];
const NEW_LINE: [char; 2] = ['\n', '\r'];
