use thiserror::Error;

#[derive(Error, Debug)]
pub enum RuntimeError {
    #[error("[{line:?}] {message:?}")]
    Syntax { message: String, line: usize },
    #[error("[{0}] INVALID BREAK, FIX IT FUCKWIT.")]
    InvalidBreak(usize),
    #[error("[{0}] SORRY MATE! YA CAN ONLY CALL FUNCTIONS, YA DAFT BUGGER!")]
    InvalidCallee(usize),
    #[error("[{0}] OI MATE! EXPECTED {1} ARGUMENTS BUT GOT {2}")]
    InvalidArity(usize, u8, usize),
    #[error("{0}")]
    General(String),
}

impl RuntimeError {
    pub fn new_syntax<T: Into<String>>(msg: T, line: usize) -> RuntimeError {
        RuntimeError::Syntax {
            message: msg.into(),
            line,
        }
    }
}
