use crate::{runtime::Value, token::Token};

use super::{
    op::{BinaryOp, UnaryOp},
    LogicalOp, Var,
};

#[derive(Clone, Debug, PartialEq)]
pub struct ExprNode {
    expr: Expr,
    line: usize,
}

impl ExprNode {
    pub fn new(expr: Expr, line: usize) -> ExprNode {
        Self { expr, line }
    }

    pub fn line(&self) -> usize {
        self.line
    }

    pub fn expr(&self) -> &Expr {
        &self.expr
    }

    pub fn literal(&self) -> Option<&Value> {
        self.expr.literal()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
    Unary(UnaryOp, Box<ExprNode>),
    Binary(Box<ExprNode>, BinaryOp, Box<ExprNode>),
    Logical(Box<ExprNode>, LogicalOp, Box<ExprNode>),
    Grouping(Box<ExprNode>),
    Literal(Value),
    Var(Var),
    Assign(Var, Box<ExprNode>),
    Call(Box<ExprNode>, Token, Vec<ExprNode>),
}

impl Expr {
    pub fn literal(&self) -> Option<&Value> {
        match self {
            Self::Literal(val) => Some(val),
            _ => None,
        }
    }
}
