pub enum ExitKind {
    Break,
    Return,
}

pub type Exit = Option<ExitKind>;
