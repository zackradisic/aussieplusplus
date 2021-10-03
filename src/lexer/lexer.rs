use anyhow::{Error, Result};
use thiserror::Error;

use crate::token::{Kind, Token};

use super::source::Source;

pub struct Lexer<T>
where
    T: Source + Iterator<Item = char>,
{
    src: T,
    line: usize,
}

impl<'a, T: Source + Iterator<Item = char>> Lexer<T> {
    pub fn new(src: T) -> Self {
        Self { src, line: 1 }
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
                if self.peek_adv('=') {
                    Kind::Equals
                } else {
                    Kind::Assign
                }
            }
            '/' => {
                if self.peek_adv('/') {
                    self.eat_line();
                    return self.next_token();
                } else {
                    Kind::Slash
                }
            }
            '!' => {
                if self.peek_adv('=') {
                    Kind::BangEqual
                } else {
                    Kind::Bang
                }
            }
            '<' => {
                if self.peek_adv('=') {
                    Kind::LTE
                } else {
                    Kind::LeftBoomerang
                }
            }
            '>' => {
                if self.peek_adv('=') {
                    Kind::GTE
                } else {
                    Kind::RightBoomerang
                }
            }
            '&' => {
                if self.peek_adv('&') {
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
                if self.peek_adv('|') {
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
                        self.eat_keyword_or_ident(c, Kind::ChookBickey)?
                    } else {
                        self.eat_identifier(c)?
                    }
                }
                'w' => {
                    if self.peek_is('a') {
                        self.eat_keyword_or_ident(c, Kind::Walkabout)?
                    } else {
                        self.eat_identifier(c)?
                    }
                }
                'b' => {
                    if self.peek_is('l') {
                        self.eat_keyword_or_ident(c, Kind::BlimeyMate)?
                    } else if self.peek_is('a') {
                        self.eat_keyword_or_ident(c, Kind::Bail)?
                    } else {
                        self.eat_identifier(c)?
                    }
                }
                'i' => {
                    if self.peek_is(' ') {
                        self.eat_keyword_or_ident(c, Kind::IRecon)?
                    } else {
                        self.eat_identifier(c)?
                    }
                }
                'y' => {
                    if self.peek_is('a') {
                        self.eat_keyword_or_ident(c, Kind::YaRecon)?
                    } else if self.peek_is('e') {
                        self.eat_keyword_or_ident(c, Kind::YeahNah)?
                    } else {
                        self.eat_identifier(c)?
                    }
                }
                'h' => {
                    if self.peek_is('a') {
                        self.eat_keyword_or_ident(c, Kind::HardYakkaFor)?
                    } else {
                        self.eat_identifier(c)?
                    }
                }
                'n' => {
                    if self.peek_is('a') {
                        self.eat_keyword_or_ident(c, Kind::NahYeah)?
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

    fn eat_keyword_or_ident(&mut self, first: char, kind: Kind) -> Result<Kind> {
        let res: Result<Kind> = match self.eat_keyword(kind) {
            Err(_) => self.eat_identifier(first),
            Ok(kind) => Ok(kind),
        };
        res
    }

    fn eat_keyword(&mut self, kind: Kind) -> Result<Kind> {
        let s: String = kind.literal().chars().skip(1).collect();
        let len = s.len();

        let mut ret: Option<Result<Kind>> = None;
        let mut expected: char;

        for i in 0..len {
            expected = s.chars().nth(i).unwrap();
            match self.peek_multi() {
                None => {
                    self.src.reset_peek();
                    ret = Some(Err(
                        LexError::ExpectedCharacter(expected, '\0', self.line).new()
                    ));
                    break;
                }
                Some(c) => {
                    if c.to_ascii_lowercase().ne(&expected) {
                        self.src.reset_peek();
                        ret = Some(Err(
                            LexError::ExpectedCharacter(expected, c, self.line).new()
                        ));
                        break;
                    }
                }
            };
        }

        if let Some(e) = ret {
            e
        } else {
            // Space, new-line, or semi-colon must separate token
            self.expect_separator()?;
            for _ in 0..len {
                let _ = self.src.next();
            }
            Ok(kind)
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

    fn peek_adv(&mut self, c: char) -> bool {
        match self.peek() {
            Some(ch) => {
                if ch == c {
                    let _ = self.next();
                    true
                } else {
                    false
                }
            }
            None => false,
        }
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
