use crate::token::Token;

use super::{ExprNode, FnDecl, ForLoop, Ident, If, Match, WhileLoop};

#[derive(Clone, Debug, PartialEq)]
pub enum Stmt {
    Expr(ExprNode),
    Block(Vec<Stmt>),
    If(If),
    Match(Match),
    VarDecl(Ident, Option<ExprNode>),
    FnDecl(FnDecl),
    Print(ExprNode),
    For(Box<ForLoop>),
    Break(Token),
    While(Box<WhileLoop>),
    Return(Option<ExprNode>),
    Import(Ident),
    Exit,
}

impl From<Stmt> for Vec<Stmt> {
    fn from(s: Stmt) -> Self {
        match s {
            Stmt::Block(stmts) => stmts,
            stmt => vec![stmt],
        }
    }
}
