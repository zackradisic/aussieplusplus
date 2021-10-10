use thiserror::Error;

#[derive(Error, Debug)]
pub enum RuntimeError {
    #[error("[{line:?}] {message:?}")]
    Syntax { message: String, line: usize },
    #[error("[{0}] Invalid break")]
    InvalidBreak(usize),
    #[error("[{0}] can only call functions and classes")]
    InvalidCallee(usize),
    #[error("[{0}] expected {1} arguments but got {2}")]
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
