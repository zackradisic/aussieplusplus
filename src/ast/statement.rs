use super::{ExprNode, Ident, Match};

#[derive(Clone, Debug, PartialEq)]
pub enum Stmt {
    Expr(ExprNode),
    Block(Vec<Stmt>),
    If(ExprNode, Box<Stmt>),
    Match(Match),
    VarDecl(Ident, Option<ExprNode>),
    Print(ExprNode),
}

impl From<Stmt> for Vec<Stmt> {
    fn from(s: Stmt) -> Self {
        match s {
            Stmt::Block(stmts) => stmts,
            stmt => vec![stmt],
        }
    }
}
