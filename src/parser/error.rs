use thiserror::Error;

use crate::token::{Kind, Token};

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("[line {0}] {}")]
    Any(usize, String),
    #[error("[line {2}] expected {0} but got {1}")]
    UnexpectedToken(Kind, Kind, usize),
    #[error("[line {2}] expected {0:?} but got {1}")]
    ExpectedTokens(Vec<Kind>, Kind, usize),
    #[error("[line {}] missing expr ended with {}", .0.line(), .0.kind())]
    MissingExpr(Token),
    #[error("[line {}] invalid assignment {}", .0.line(), .0.kind())]
    InvalidAssigment(Token),
    #[error("[line {0}] too many function arguments (max 255)")]
    TooManyArguments(usize),
    #[error("[line {0}] too many default branches in match statement")]
    TooManyMatchDefaultBranches(usize),
    #[error("[line {0}] invalid range {1} {2}")]
    InvalidRange(usize, String, String),
}
