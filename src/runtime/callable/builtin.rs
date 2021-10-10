use std::fmt::Display;
use std::rc::Rc;
use std::{thread, time::Duration};

use crate::runtime::error::RuntimeError;
use crate::runtime::{Interpreter, Value};

use super::AussieCallable;

#[derive(Clone, PartialEq, Debug)]
pub enum BuiltIn {
    Sleep(Sleep),
}

impl AussieCallable for BuiltIn {
    fn call(&self, interpreter: &mut Interpreter, args: &Vec<Value>) -> anyhow::Result<Value> {
        match self {
            Self::Sleep(clock) => clock.call(interpreter, args),
        }
    }

    fn arity(&self) -> u8 {
        match self {
            Self::Sleep(clock) => clock.arity(),
        }
    }

    fn name(&self) -> Rc<String> {
        match self {
            Self::Sleep(clock) => clock.name(),
        }
    }
}

impl Display for BuiltIn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Sleep(_) => write!(f, "clock()"),
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct Sleep {
    name: Rc<String>,
}

impl AussieCallable for Sleep {
    fn call(&self, _: &mut Interpreter, args: &Vec<Value>) -> anyhow::Result<Value> {
        let duration = match &args[0] {
            Value::Number(n) => *n,
            _ => return Err(RuntimeError::General("expected a number".into()).into()),
        };

        if duration < 0.0 {
            return Err(RuntimeError::General("expected a number > 0".into()).into());
        }

        thread::sleep(Duration::from_millis(duration as u64));

        Ok(Value::Nil)
    }

    fn arity(&self) -> u8 {
        1
    }

    fn name(&self) -> Rc<String> {
        self.name.clone()
    }
}
