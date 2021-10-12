use thiserror::Error;

use crate::token::{Kind, Token};

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("[line {0}] {1}")]
    Any(usize, String),
    #[error("[line {2}] OI MATE! EXPECTED {0} BUT GOT '{1}'")]
    UnexpectedToken(Kind, Kind, usize),
    #[error("[line {2}] OI MATE! EXPECTED {0:?} BUT GOT '{1}'")]
    ExpectedTokens(Vec<Kind>, Kind, usize),
    #[error("[line {}] MISSING EXPR ENDED WITH '{}'", .0.line(), .0.kind())]
    MissingExpr(Token),
    #[error("[line {}] HEY FUCKWIT! WHY YA FACKIN TRYNA ASSIGN TO A {}", .0.line(), .0.kind())]
    InvalidAssigment(Token),
    #[error("[line {0}] MATE, THAT'S TOO MANY ARGUMENTS (max 255)")]
    TooManyArguments(usize),
    #[error("[line {0}] TOO MANY DEFAULT BRANCHES IN MATCH STATEMENT, YA DAFT BUGGER")]
    TooManyMatchDefaultBranches(usize),
    #[error("[line {0}] CAN YA FUKING COUNT, MATE? INVALID RANGE {1} {2}")]
    InvalidRange(usize, String, String),
    #[error("[line {0}] EXPECTED NUMBER, STRING, BOOLEAN, NIL, OR IDENTIFIER BUT GOT '{1}'")]
    ExpectPrimary(usize, Kind),
    #[error("YA DAFT BUGGER! YA DIDN'T WRITE \"G'DAY MATE!\" TO START PROGRAM!!")]
    ExpectProgramStart,
}
