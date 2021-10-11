use super::{ExprNode, Stmt, Var};

#[derive(Clone, Debug, PartialEq)]
pub struct ForLoop {
    pub var: Var,
    pub range: (RangeBound<ExprNode>, RangeBound<ExprNode>),
    pub body: Vec<Stmt>,
}

impl ForLoop {
    pub fn new(
        var: Var,
        range: (RangeBound<ExprNode>, RangeBound<ExprNode>),
        body: Vec<Stmt>,
    ) -> Self {
        Self { var, range, body }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct WhileLoop {
    pub var: Var,
    pub cond: ExprNode,
    pub body: Vec<Stmt>,
}

impl WhileLoop {
    pub fn new(var: Var, cond: ExprNode, body: Vec<Stmt>) -> Self {
        Self { var, cond, body }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum RangeBound<T> {
    Inclusive(T),
    Exclusive(T),
}

impl RangeBound<ExprNode> {
    pub fn expr(&self) -> &ExprNode {
        match self {
            Self::Inclusive(expr) => expr,
            Self::Exclusive(expr) => expr,
        }
    }

    pub fn to_evaluated(&self, val: f64) -> RangeBound<f64> {
        match self {
            Self::Inclusive(_) => RangeBound::Inclusive(val),
            Self::Exclusive(_) => RangeBound::Exclusive(val),
        }
    }
}

impl RangeBound<f64> {
    pub fn value(&self) -> f64 {
        match self {
            Self::Inclusive(val) => *val,
            Self::Exclusive(val) => *val,
        }
    }
}

pub trait Range<T> {
    fn satisfied(&self, i: T) -> bool;
    fn values(&self) -> (T, T);
    fn iterate(&self, i: &mut T);
}

impl Range<f64> for (RangeBound<f64>, RangeBound<f64>) {
    fn iterate(&self, val: &mut f64) {
        if self.0.value() < self.1.value() {
            *val += 1f64;
        } else if self.0.value() > self.1.value() {
            *val -= 1f64;
        }
    }

    fn satisfied(&self, i: f64) -> bool {
        match self {
            (RangeBound::Inclusive(start), RangeBound::Inclusive(end)) => {
                if start < end {
                    i >= *start && i <= *end
                } else if start > end {
                    i >= *end && i <= *start
                } else {
                    let error_margin = f64::EPSILON;
                    (i - *start).abs() < error_margin
                }
            }
            (RangeBound::Inclusive(start), RangeBound::Exclusive(end)) => {
                if start < end {
                    i >= *start && i < *end
                } else if start > end {
                    i < *end && i >= *start
                } else {
                    let error_margin = f64::EPSILON;
                    (i - *start).abs() < error_margin
                }
            }
            (RangeBound::Exclusive(start), RangeBound::Inclusive(end)) => {
                if start < end {
                    i > *start && i <= *end
                } else if end < start {
                    i >= *end && i < *start
                } else {
                    let error_margin = f64::EPSILON;
                    (i - *end).abs() < error_margin
                }
            }
            (RangeBound::Exclusive(start), RangeBound::Exclusive(end)) => {
                if start < end {
                    i > *start && i < *end
                } else if end < start {
                    i > *end && i < *start
                } else {
                    false
                }
            }
        }
    }

    fn values(&self) -> (f64, f64) {
        match self {
            (RangeBound::Inclusive(a), RangeBound::Inclusive(b)) => (*a, *b),
            (RangeBound::Inclusive(a), RangeBound::Exclusive(b)) => {
                if a < b {
                    (*a, *b + 1f64)
                } else {
                    (*a, *b - 1f64)
                }
            }
            (RangeBound::Exclusive(a), RangeBound::Exclusive(b)) => {
                if a < b {
                    (*a + 1f64, *b - 1f64)
                } else {
                    (*a - 1f64, *b + 1f64)
                }
            }
            (RangeBound::Exclusive(a), RangeBound::Inclusive(b)) => {
                if a < b {
                    (*a + 1f64, *b)
                } else {
                    (*a - 1f64, *b)
                }
            }
        }
    }
}
