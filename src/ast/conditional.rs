use crate::{
    runtime::{RuntimePartialEq, Value},
    token::{Kind, Token},
};

use super::{ExprNode, Ident, Stmt, Var};

#[derive(Clone, Debug, PartialEq)]
pub struct If {
    pub cond: ExprNode,
    pub then: Box<Stmt>,
    pub else_: Option<Box<Stmt>>,
}

impl If {
    pub fn new(cond: ExprNode, then: Box<Stmt>, else_: Option<Box<Stmt>>) -> Self {
        Self { cond, then, else_ }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Match {
    pub val: ExprNode,
    pub branches: Vec<MatchBranch>,
    pub default: Option<MatchBranch>,
}

impl Match {
    pub fn new(val: ExprNode, branches: Vec<MatchBranch>, default: Option<MatchBranch>) -> Match {
        Match {
            val,
            branches,
            default,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct MatchBranch {
    pub pat: Pattern,
    pub body: Vec<Stmt>,
    line: usize,
}

impl MatchBranch {
    pub fn new(pat: Pattern, body: Vec<Stmt>, line: usize) -> Self {
        Self { pat, body, line }
    }

    pub fn line(&self) -> usize {
        self.line
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Pattern {
    Var(Var),
    String(String),
    Number(f64),
    Bool(bool),
    Nil,
}

impl RuntimePartialEq<Value> for Pattern {
    fn runtime_eq(&self, other: &Value) -> bool {
        match (self, other) {
            (Self::String(l), Value::String(r)) => l == r,
            (Self::Number(l), Value::Number(r)) => l == r,
            (Self::Bool(l), Value::Bool(r)) => l == r,
            (Self::Nil, Value::Nil) => true,
            _ => false,
        }
    }

    fn runtime_ne(&self, other: &Value) -> bool {
        !self.runtime_eq(other)
    }
}

impl From<Kind> for Option<Pattern> {
    fn from(kind: Kind) -> Self {
        match &kind {
            Kind::Number(n) => Some(Pattern::Number(*n)),
            Kind::String(s) => Some(Pattern::String(s.clone())),
            Kind::True => Some(Pattern::Bool(true)),
            Kind::False => Some(Pattern::Bool(false)),
            Kind::BuggerAll => Some(Pattern::Nil),
            Kind::Ident(ident) => Some(Pattern::Var(Var::new(
                Ident::new(ident.clone(), 0),
                usize::MAX,
            ))),
            _ => None,
        }
    }
}

impl From<Token> for Option<Pattern> {
    fn from(tok: Token) -> Self {
        match &tok.kind {
            Kind::Number(n) => Some(Pattern::Number(*n)),
            Kind::String(s) => Some(Pattern::String(s.clone())),
            Kind::True => Some(Pattern::Bool(true)),
            Kind::False => Some(Pattern::Bool(false)),
            Kind::BuggerAll => Some(Pattern::Nil),
            Kind::Ident(ident) => Some(Pattern::Var(Var::new(
                Ident::new(ident.clone(), tok.line()),
                usize::MAX,
            ))),
            _ => None,
        }
    }
}
