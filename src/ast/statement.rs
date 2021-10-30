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
    Return(Token, Option<ExprNode>),
    Import(Ident),
    Exit(bool),
}

impl From<Stmt> for Vec<Stmt> {
    fn from(s: Stmt) -> Self {
        match s {
            Stmt::Block(stmts) => stmts,
            stmt => vec![stmt],
        }
    }
}

impl Stmt {
    pub fn kind(&self) -> String {
        match self {
            Self::Expr(_) => "expr",
            Self::Block(_) => "block",
            Self::If(_) => "if",
            Self::Match(_) => "match",
            Self::VarDecl(_, _) => "var decl",
            Self::FnDecl(_) => "fn decl",
            Self::Print(_) => "print",
            Self::For(_) => "for",
            Self::Break(_) => "rbreak",
            Self::While(_) => "while",
            Self::Return(_, _) => "return",
            Self::Import(_) => "import",
            Self::Exit(_) => "exit",
        }
        .into()
    }
}
