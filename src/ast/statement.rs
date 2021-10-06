use super::{ExprNode, Ident, MatchBranch};

#[derive(Clone, Debug, PartialEq)]
pub enum Stmt {
    Expr(ExprNode),
    Block(Vec<Stmt>),
    If(ExprNode, Box<Stmt>),
    Match(ExprNode, Vec<MatchBranch>),
    VarDecl(Ident, Option<ExprNode>),
}
