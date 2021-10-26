use thiserror::Error;

#[derive(Error, Debug)]
pub enum RuntimeError {
    #[error("[{0}] {1}")]
    Syntax(usize, String),
    #[error("[{0}] INVALID BREAK, FIX IT FUCKWIT.")]
    InvalidBreak(usize),
    #[error("[{0}] SORRY MATE! YA CAN ONLY CALL FUNCTIONS, YA DAFT BUGGER!")]
    InvalidCallee(usize),
    #[error("[{0}] OI MATE, CAN YA FUCKIN' COUNT?? EXPECTED {1} ARGUMENTS BUT GOT {2}")]
    InvalidArity(usize, u8, usize),
    #[error("[{0}] CAN'T FIND THE IMPORT {1}")]
    UnknownImport(usize, String),
    #[error("{0}")]
    General(String),
    #[error("[{0}] SORRY C***! '{1}' ISN'T DEFINED, YA DAFT BUGGER!")]
    UndefinedVariable(usize, String),
}

impl RuntimeError {
    pub fn new_syntax<T: Into<String>>(msg: T, line: usize) -> RuntimeError {
        RuntimeError::Syntax(line, msg.into())
    }
}
