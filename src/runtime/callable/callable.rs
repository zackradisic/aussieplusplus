use std::{fmt::Display, rc::Rc};

use anyhow::Result;

use crate::runtime::{Interpreter, Value};

use super::Function;

pub trait AussieCallable {
    fn call(&self, interpreter: &mut Interpreter, args: &Vec<Value>) -> Result<Value>;
    fn arity(&self) -> u8;
    fn name(&self) -> Rc<String>;
}

#[derive(Clone, PartialEq, Debug)]
pub enum Callable {
    Function(Function),
}

impl AussieCallable for Callable {
    fn call(&self, interpreter: &mut Interpreter, args: &Vec<Value>) -> Result<Value> {
        match self {
            Callable::Function(func) => func.call(interpreter, args),
        }
    }

    fn arity(&self) -> u8 {
        match self {
            Callable::Function(func) => func.arity(),
        }
    }

    fn name(&self) -> Rc<String> {
        match self {
            Callable::Function(func) => func.name(),
        }
    }
}

impl Display for Callable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Function(func) => func.fmt(f),
        }
    }
}
