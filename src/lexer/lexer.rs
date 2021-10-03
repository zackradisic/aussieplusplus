use anyhow::{Error, Result};
use std::str::Chars;
use thiserror::Error;

use itertools::{multipeek, MultiPeek};

use crate::{
    eat_keyword, eat_keyword_or_ident, peek_adv,
    token::{Kind, Token},
};

pub struct Lexer<'a> {
    src: MultiPeek<Chars<'a>>,
    line: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(src: Chars<'a>) -> Self {
        Self {
            src: multipeek(src),
            line: 1,
        }
    }

    pub fn lex(&mut self) -> (Vec<Token>, bool) {
        let mut tokens: Vec<Token> = Vec::new();
        let mut had_error = false;

        loop {
            match self.next_token() {
                Ok(tok) => {
                    tokens.push(tok.clone());
                    match tok.kind() {
                        Kind::ChookBickey | Kind::EOF => break,
                        _ => {}
                    }
                }
                Err(e) => {
                    had_error = true;
                    eprintln!("{}", e);
                }
            }
        }

        (tokens, had_error)
    }

    fn next_token(&mut self) -> Result<Token> {
        self.eat_whitespace();

        let ch = match self.next() {
            Some(ch) => ch,
            None => return Ok(Token::new(Kind::EOF, self.line)),
        };

        let kind: Kind = match ch {
            '(' => Kind::LeftParen,
            ')' => Kind::RightParen,
            ',' => Kind::Comma,
            '+' => Kind::Plus,
            '-' => Kind::Minus,
            '*' => Kind::Asterisk,
            ';' => Kind::Semicolon,

            '=' => {
                if peek_adv!(self, '=') {
                    Kind::Equals
                } else {
                    Kind::Assign
                }
            }
            '/' => {
                if peek_adv!(self, '/') {
                    self.eat_line();
                    return self.next_token();
                } else {
                    Kind::Slash
                }
            }
            '!' => {
                if peek_adv!(self, '=') {
                    Kind::BangEqual
                } else {
                    Kind::Bang
                }
            }
            '<' => {
                if peek_adv!(self, '=') {
                    Kind::LTE
                } else {
                    Kind::LeftBoomerang
                }
            }
            '>' => {
                if peek_adv!(self, '=') {
                    Kind::GTE
                } else {
                    Kind::RightBoomerang
                }
            }
            '&' => {
                if peek_adv!(self, '&') {
                    Kind::And
                } else {
                    return Err(LexError::ExpectedCharacter(
                        '&',
                        self.peek().unwrap_or_default(),
                        self.line,
                    )
                    .new());
                }
            }
            '|' => {
                if peek_adv!(self, '|') {
                    Kind::Or
                } else {
                    return Err(LexError::ExpectedCharacter(
                        '|',
                        self.peek().unwrap_or_default(),
                        self.line,
                    )
                    .new());
                }
            }
            c => match c.to_ascii_lowercase() {
                'c' => {
                    if self.peek_is('h') {
                        eat_keyword_or_ident!(self, c, Kind::ChookBickey)?
                    } else {
                        self.eat_identifier(c)?
                    }
                }
                'w' => {
                    if self.peek_is('a') {
                        eat_keyword_or_ident!(self, c, Kind::Walkabout)?
                    } else {
                        self.eat_identifier(c)?
                    }
                }
                'b' => {
                    if self.peek_is('l') {
                        eat_keyword_or_ident!(self, c, Kind::BlimeyMate)?
                    } else if self.peek_is('a') {
                        eat_keyword_or_ident!(self, c, Kind::Bail)?
                    } else {
                        self.eat_identifier(c)?
                    }
                }
                'i' => {
                    if self.peek_is(' ') {
                        eat_keyword_or_ident!(self, c, Kind::IRecon)?
                    } else {
                        self.eat_identifier(c)?
                    }
                }
                'y' => {
                    if self.peek_is('a') {
                        eat_keyword_or_ident!(self, c, Kind::YaRecon)?
                    } else if self.peek_is('e') {
                        eat_keyword_or_ident!(self, c, Kind::YeahNah)?
                    } else {
                        self.eat_identifier(c)?
                    }
                }
                'h' => {
                    if self.peek_is('a') {
                        eat_keyword_or_ident!(self, c, Kind::HardYakkaFor)?
                    } else {
                        self.eat_identifier(c)?
                    }
                }
                'n' => {
                    if self.peek_is('a') {
                        eat_keyword_or_ident!(self, c, Kind::NahYeah)?
                    } else {
                        self.eat_identifier(c)?
                    }
                }
                '"' => self.eat_string()?,
                _ => {
                    if c.is_digit(10) {
                        self.eat_number(c)?
                    } else if c == '_' || c.is_alphabetic() {
                        self.eat_identifier(c)?
                    } else {
                        return Err(LexError::UnexpectedCharacter(c, self.line).new());
                    }
                }
            },
        };

        Ok(Token::new(kind, self.line))
    }

    fn eat_number(&mut self, first: char) -> Result<Kind> {
        let mut s = String::from(first);
        let mut has_decimal = false;

        while let Some(peek) = self.peek() {
            if peek.is_digit(10) {
                let _ = self.next();
                s.push(peek);
            } else if peek == '.' {
                if has_decimal {
                    return Err(LexError::InvalidNumber(self.line).new());
                }
                has_decimal = true;
                s.push(peek);
                let _ = self.next();
            } else {
                break;
            }
        }

        if let Ok(f) = s.parse::<f64>() {
            return Ok(Kind::Number(f));
        }

        return Err(LexError::InvalidNumber(self.line).new());
    }

    fn eat_string(&mut self) -> Result<Kind> {
        let mut s = String::new();
        let mut ended = false;

        while let Some(next) = self.next() {
            match next {
                '"' => {
                    ended = true;
                    break;
                }
                ch => {
                    if ch == '\n' {
                        self.line += 1;
                    }
                    s.push(ch);
                }
            }
        }

        if !ended {
            return Err(LexError::UnterminatedString(self.line).new());
        }

        Ok(Kind::String(s))
    }

    fn eat_identifier(&mut self, first: char) -> Result<Kind> {
        let mut s: String = first.into();
        loop {
            match self.peek() {
                None => break,
                Some(c) => {
                    if c.is_digit(10) || c.is_alphabetic() || c == '_' {
                        s.push(c);
                        let _ = self.next();
                    } else {
                        break;
                    }
                }
            }
        }

        // Space, new-line, or semi-colon must separate token
        self.expect_separator()?;

        Ok(Kind::Ident(s))
    }

    fn expect_separator(&mut self) -> Result<()> {
        let separated = match self.peek() {
            Some(' ' | '\n' | ';') => true,
            // EOF counts as delineator
            None => true,
            _ => false,
        };

        if separated {
            return Ok(());
        }

        return Err(LexError::ExpectedCharacters(
            vec![' ', '\n', ';'],
            self.peek().unwrap_or_default(),
            self.line,
        )
        .new());
    }

    fn eat_whitespace(&mut self) {
        while let Some(peek) = self.peek() {
            if peek == '\n' {
                self.line += 1;
                let _ = self.next();
            } else if peek.is_whitespace() {
                let _ = self.next();
            } else {
                break;
            }
        }
    }

    fn eat_line(&mut self) {
        while let Some(c) = self.peek() {
            let _ = self.next();
            if c == '\n' {
                self.line += 1;
                break;
            }
        }
    }

    fn next(&mut self) -> Option<char> {
        self.src.next()
    }

    fn peek(&mut self) -> Option<char> {
        let res = self.src.peek().map(|&p| p);
        self.src.reset_peek();
        res
    }

    fn peek_is(&mut self, c: char) -> bool {
        match self.peek() {
            Some(ch) => {
                if ch.to_ascii_lowercase().eq(&c) {
                    true
                } else {
                    false
                }
            }
            None => false,
        }
    }

    /// Can be called multiple times to peek more than one
    /// character ahead
    fn peek_multi(&mut self) -> Option<char> {
        self.src.peek().map(|&p| p)
    }
}

#[derive(Error, Debug)]
pub enum LexError {
    #[error("[line {2}] expected {0} but got {1}")]
    ExpectedCharacter(char, char, usize),
    #[error("[line {2}] expected one of {0:?} but got {1}")]
    ExpectedCharacters(Vec<char>, char, usize),
    #[error("[line {0}] unexpected EOF")]
    UnexpectedEOF(usize),
    #[error("[line {1}] unexpected character {0}")]
    UnexpectedCharacter(char, usize),
    #[error("[line {0}] unterminated")]
    UnterminatedString(usize),
    #[error("[line {0}] invalid number")]
    InvalidNumber(usize),
}

impl LexError {
    pub fn new(self) -> Error {
        Error::new(self)
    }
}
