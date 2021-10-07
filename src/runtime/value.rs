use std::fmt::Display;

use crate::token::Kind;

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    String(String),
    Number(f64),
    Bool(bool),
    Nil,
}

impl From<Value> for String {
    fn from(val: Value) -> Self {
        match val {
            Value::Bool(b) => {
                if b {
                    "Nah, yeah".into()
                } else {
                    "Yeah, nah".into()
                }
            }
            Value::Nil => format!("{}", Kind::BuggerAll),
            Value::Number(n) => format!("{}", n),
            Value::String(s) => s.clone(),
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s: String = self.clone().into();
        write!(f, "{}", s)
    }
}

impl From<&str> for Value {
    fn from(s: &str) -> Self {
        let s: String = s.into();
        s.into()
    }
}

impl From<String> for Value {
    fn from(s: String) -> Self {
        Value::String(s)
    }
}

impl From<bool> for Value {
    fn from(b: bool) -> Self {
        Value::Bool(b)
    }
}

// impl From<f64> for Value {
//     fn from(b: f64) -> Self {
//         Value::Number(b)
//     }
// }

impl<T: Into<f64> + Numeric> From<T> for Value {
    fn from(num: T) -> Self {
        Value::Number(num.into())
    }
}

pub trait Numeric {}
impl Numeric for f64 {}
impl Numeric for f32 {}
impl Numeric for i64 {}
impl Numeric for i32 {}
impl Numeric for i16 {}
impl Numeric for i8 {}
impl Numeric for isize {}
impl Numeric for u64 {}
impl Numeric for u32 {}
impl Numeric for u16 {}
impl Numeric for u8 {}
impl Numeric for usize {}
