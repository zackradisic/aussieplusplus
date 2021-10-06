use crate::token::Kind;

use super::{ExprNode, Stmt, Var};

#[derive(Clone, Debug, PartialEq)]
pub struct MatchBranch {
    pat: Pattern,
    body: MatchBody,
    line: usize,
}

impl MatchBranch {
    pub fn new(pat: Pattern, body: MatchBody, line: usize) -> Self {
        Self { pat, body, line }
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

impl From<Kind> for Option<Pattern> {
    fn from(kind: Kind) -> Self {
        match kind {
            Kind::Number(n) => Some(Pattern::Number(n)),
            Kind::String(s) => Some(Pattern::String(s)),
            Kind::NahYeah => Some(Pattern::Bool(true)),
            Kind::YeahNah => Some(Pattern::Bool(false)),
            Kind::BuggerAll => Some(Pattern::Nil),
            _ => None,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MatchBody {
    Expr(ExprNode),
    Block(Vec<Stmt>),
}

impl From<Stmt> for Option<MatchBody> {
    fn from(s: Stmt) -> Self {
        match s {
            Stmt::Block(stmts) => Some(MatchBody::Block(stmts)),
            Stmt::Expr(expr) => Some(MatchBody::Expr(expr)),
            _ => None,
        }
    }
}
