use std::fmt::Display;

use crate::token::Kind;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum UnaryOp {
    Bang,
    Minus,
}

impl From<Kind> for Option<UnaryOp> {
    fn from(kind: Kind) -> Self {
        match kind {
            Kind::Bang => Some(UnaryOp::Bang),
            Kind::Minus => Some(UnaryOp::Minus),
            _ => None,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum BinaryOp {
    Plus,
    Minus,
    Multiply,
    Divide,
    Equal,
    NotEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
}

impl From<Kind> for Option<BinaryOp> {
    fn from(kind: Kind) -> Self {
        match kind {
            Kind::Plus => Some(BinaryOp::Plus),
            Kind::Minus => Some(BinaryOp::Minus),
            Kind::Asterisk => Some(BinaryOp::Multiply),
            Kind::Slash => Some(BinaryOp::Divide),
            Kind::Equals => Some(BinaryOp::Equal),
            Kind::BangEqual => Some(BinaryOp::NotEqual),
            Kind::LeftBoomerang => Some(BinaryOp::Less),
            Kind::LTE => Some(BinaryOp::LessEqual),
            Kind::RightBoomerang => Some(BinaryOp::Greater),
            Kind::GTE => Some(BinaryOp::GreaterEqual),
            _ => None,
        }
    }
}

impl Display for BinaryOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Plus => write!(f, "+"),
            Self::Minus => write!(f, "-"),
            Self::Multiply => write!(f, "*"),
            Self::Divide => write!(f, "/"),
            Self::Equal => write!(f, "=="),
            Self::NotEqual => write!(f, "!="),
            Self::Less => write!(f, "<"),
            Self::LessEqual => write!(f, "<="),
            Self::Greater => write!(f, ">"),
            Self::GreaterEqual => write!(f, ">="),
        }
    }
}
