use anyhow::Result;
use thiserror::Error;

use crate::token::{Kind, Token};

use super::source::Source;

pub struct Lexer<T>
where
    T: Source,
{
    src: T,
    line: usize,
}

impl<'a, T: Source> Lexer<T> {
    pub fn new(src: T) -> Self {
        Self { src, line: 1 }
    }

    pub fn lex(&mut self) -> (Vec<Token>, bool) {
        let mut tokens: Vec<Token> = Vec::new();
        let mut had_error = false;

        let mut last_nah_yeah: Option<Kind> = None;
        let mut nah_yeah_count = 0;

        loop {
            match self.next_token() {
                Ok(tok) => {
                    match tok.kind() {
                        Kind::EOF => {
                            tokens.push(tok);
                            break;
                        }
                        Kind::Cheers => {
                            tokens.push(Token::new(Kind::Cheers, tok.line()));
                            tokens.push(Token::new(Kind::EOF, tok.line() + 1));
                            break;
                        }
                        Kind::Nah | Kind::Yeah => {
                            last_nah_yeah = Some(tok.kind());
                            nah_yeah_count += 1;
                            // Don't fall through
                            continue;
                        }
                        Kind::Bang => {
                            if let Some(k) = last_nah_yeah {
                                let tok = match k {
                                    Kind::Yeah => Token::new(Kind::True, tok.line()),
                                    Kind::Nah => Token::new(Kind::False, tok.line()),
                                    _ => panic!("This should not happen"),
                                };

                                if nah_yeah_count < 2 {
                                    had_error = true;
                                    eprintln!("{}", LexError::TooLittleNahYeahs(tok.line()));
                                }

                                last_nah_yeah = None;
                                nah_yeah_count = 0;
                                tokens.push(tok);

                                continue;
                            }
                        }
                        _ => {}
                    }

                    if last_nah_yeah.is_some() {
                        had_error = true;
                        eprintln!(
                            "{}",
                            LexError::Expected("!".into(), tok.kind().literal(), tok.line())
                        );
                    }

                    tokens.push(tok);
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
            '%' => Kind::Modulo,
            '~' => Kind::Tilde,
            '?' => Kind::QuestionMark,
            '[' => Kind::LeftBracket,
            ']' => Kind::RightBracket,
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
                    .into());
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
                    .into());
                }
            }
            c => match c.to_ascii_lowercase() {
                'f' if self.peek_is('u') => self.eat_keyword_or_ident(c, Kind::FuckinPiker)?,
                'm' if self.peek_is('a') => self.eat_keyword_or_ident(c, Kind::MateFuckThis)?,
                'u' if self.peek_is('n') => self.eat_keyword_or_ident(c, Kind::Until)?,
                't' if self.peek_is('o') => self.eat_keyword_or_ident(c, Kind::To)?,
                'f' if self.peek_is('r') => self.eat_keyword_or_ident(c, Kind::From)?,
                'g' if self.peek_is('i') => self.eat_keyword_or_ident(c, Kind::Gimme)?,
                'g' if self.peek_is('\'') => self.eat_keyword_or_ident(c, Kind::GdayMate)?,
                'c' if self.peek_is('h') => self.eat_keyword_or_ident(c, Kind::Cheers)?,
                'w' if self.peek_is('a') => self.eat_keyword_or_ident(c, Kind::Walkabout)?,
                'w' if self.peek_is('h') => self.eat_keyword_or_ident(c, Kind::Whatabout)?,
                't' if self.peek_is('h') => self.eat_keyword_or_ident(c, Kind::HardYakkaFor)?,
                'n' if self.peek_is('a') => self.eat_nah_or_yeah_or_ident(c, Kind::Nah)?,

                'b' => {
                    if self.peek_is('a') {
                        self.eat_keyword_or_ident(c, Kind::Bail)?
                    } else if self.peek_is('u') {
                        self.eat_keyword_or_ident(c, Kind::BuggerAll)?
                    } else {
                        self.eat_identifier(c)?
                    }
                }

                'i' => {
                    if self.peek_is('m') {
                        self.eat_keyword_or_ident(c, Kind::Import)?
                    } else if self.peek_is(' ') {
                        self.eat_keyword_or_ident(c, Kind::IReckon)?
                    } else if self.peek_is('s') {
                        match self.eat_keyword_or_ident(c, Kind::Isa)? {
                            Kind::Isa => Kind::Isa,
                            Kind::Ident(maybe_is) if maybe_is.to_ascii_lowercase() == "is" => {
                                Kind::Is
                            }
                            ident => ident,
                        }
                    } else if self.peek_is('\'') {
                        self.eat_keyword_or_ident(c, Kind::IllHaveA)?
                    } else {
                        self.eat_identifier(c)?
                    }
                }

                'y' => {
                    if self.peek_is('a') {
                        self.eat_keyword_or_ident(c, Kind::YaReckon)?
                    } else if self.peek_is('e') {
                        self.eat_nah_or_yeah_or_ident(c, Kind::Yeah)?
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
                        return Err(LexError::UnexpectedCharacter(c, self.line).into());
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
                    return Err(LexError::InvalidNumber(self.line).into());
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

        Err(LexError::InvalidNumber(self.line).into())
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
            return Err(LexError::UnterminatedString(self.line).into());
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

    fn is_separator(c: Option<char>) -> bool {
        matches!(
            c,
            Some(' ' | '\n' | ';' | ',' | '(' | ')' | '[' | ']') | None
        )
    }

    fn expect_separator(&mut self) -> Result<()> {
        if Self::is_separator(self.peek()) {
            return Ok(());
        }

        Err(LexError::ExpectedCharacters(
            vec![' ', '\n', ';', ','],
            self.peek().unwrap_or_default(),
            self.line,
        )
        .into())
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

    fn eat_nah_or_yeah_or_ident(&mut self, first: char, kind: Kind) -> Result<Kind> {
        let res: Result<Kind> = match self.eat_keyword(kind, false) {
            Err(_) => self.eat_identifier(first),
            Ok(kind) => {
                self.peek_adv(',');
                Ok(kind)
            }
        };
        res
    }

    fn eat_keyword_or_ident(&mut self, first: char, kind: Kind) -> Result<Kind> {
        let res: Result<Kind> = match self.eat_keyword(kind, true) {
            Err(_) => self.eat_identifier(first),
            Ok(kind) => Ok(kind),
        };
        res
    }

    fn eat_keyword(&mut self, kind: Kind, expect_separator: bool) -> Result<Kind> {
        let after_first: String = kind.literal().chars().skip(1).collect();

        let n = self.is_any_str(&after_first)?;

        if expect_separator {
            self.expect_separator()?;
        }

        // Eat peeked chars
        self.eat_n(n);

        Ok(kind)
    }

    fn is_any_str(&mut self, s: &str) -> Result<usize> {
        let len = s.len();

        let mut expected: char;

        for i in 0..len {
            expected = s.chars().nth(i).unwrap();
            match self.peek_multi() {
                None => {
                    self.src.reset_peek();
                    return Err(LexError::ExpectedCharacter(expected, '\0', self.line).into());
                }
                Some(c) => {
                    if c.to_ascii_lowercase().ne(&expected) {
                        self.src.reset_peek();
                        return Err(LexError::ExpectedCharacter(expected, c, self.line).into());
                    }
                }
            };
        }

        Ok(len)
    }

    fn eat_n(&mut self, n: usize) {
        for _ in 0..n {
            let _ = self.next();
        }
    }
}

// General utilities
impl<'a, T: Source> Lexer<T> {
    fn next(&mut self) -> Option<char> {
        self.src.next()
    }

    fn peek(&mut self) -> Option<char> {
        let res = self.src.peek().copied();
        self.src.reset_peek();
        res
    }

    fn peek_is(&mut self, c: char) -> bool {
        match self.peek() {
            Some(ch) => ch.to_ascii_lowercase().eq(&c),
            None => false,
        }
    }

    /// Can be called multiple times to peek more than one
    /// character ahead
    fn peek_multi(&mut self) -> Option<char> {
        self.src.peek().copied()
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
    #[error("[line {0}] OI MATE! YA NEED AT LEAST 2 'NAH's or 'YEAH's TO MAKE A BOOL!!!")]
    TooLittleNahYeahs(usize),
    #[error("[line {2}] OI MATE! expected {0} but got {1}")]
    Expected(String, String, usize),
    #[error("[line {2}] OI MATE! expected {0} but got {1}")]
    ExpectedCharacter(char, char, usize),
    #[error("[line {2}] FUCK ME DEAD! EXPECTED ONE OF {0:?} BUT GOT {1}")]
    ExpectedCharacters(Vec<char>, char, usize),
    #[error("[line {0}] STREWTH! unexpected EOF")]
    UnexpectedEOF(usize),
    #[error("[line {1}] BLOODY HELL! UNEXPECTED CHARACTER {0}")]
    UnexpectedCharacter(char, usize),
    #[error("[line {0}] UNTERMINATED STRING YA FUCKWIT!")]
    UnterminatedString(usize),
    #[error("[line {0}] OI BLUDGER! INVALID NUMBER")]
    InvalidNumber(usize),
}
