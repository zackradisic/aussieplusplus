use super::Value;

pub enum ExitKind {
    Break(usize),
    Return(Value),
}

pub type Exit = Option<ExitKind>;
