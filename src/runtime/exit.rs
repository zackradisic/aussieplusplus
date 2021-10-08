pub enum ExitKind {
    Break(usize),
    Return(usize),
}

pub type Exit = Option<ExitKind>;
