use std::fmt::Display;
use std::rc::Rc;
use std::{thread, time::Duration};

use chrono::{TimeZone, Utc};

use crate::runtime::error::RuntimeError;
use crate::runtime::{Interpreter, Value};

use super::AussieCallable;

#[derive(Clone, PartialEq, Debug)]
pub enum BuiltIn {
    Sleep(Sleep),
    Time(Time),
}

impl BuiltIn {
    pub fn lookup(name: &str) -> Option<Self> {
        match name {
            "HitTheSack" => Some(BuiltIn::Sleep(Sleep::default())),
            "GimmeTime" => Some(BuiltIn::Time(Time::default())),
            _ => None,
        }
    }
}

impl AussieCallable for BuiltIn {
    fn call(&self, interpreter: &mut Interpreter, args: &[Value]) -> anyhow::Result<Value> {
        match self {
            Self::Sleep(sleep) => sleep.call(interpreter, args),
            Self::Time(time) => time.call(interpreter, args),
        }
    }

    fn arity(&self) -> u8 {
        match self {
            Self::Sleep(sleep) => sleep.arity(),
            Self::Time(time) => time.arity(),
        }
    }

    fn name(&self) -> Rc<String> {
        match self {
            Self::Sleep(sleep) => sleep.name(),
            Self::Time(time) => time.name(),
        }
    }
}

impl Display for BuiltIn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Sleep(s) => write!(f, "{}(ms)", s.name()),
            Self::Time(t) => write!(f, "{}()", t.name()),
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct Sleep {
    name: Rc<String>,
}

impl Default for Sleep {
    fn default() -> Self {
        Self {
            name: Rc::new("HitTheSack".into()),
        }
    }
}

impl AussieCallable for Sleep {
    fn call(&self, _: &mut Interpreter, args: &[Value]) -> anyhow::Result<Value> {
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

#[derive(Clone, PartialEq, Debug)]
pub struct Time {
    name: Rc<String>,
}

impl Default for Time {
    fn default() -> Self {
        Self {
            name: Rc::new("GimmeTime".into()),
        }
    }
}

impl AussieCallable for Time {
    fn call(&self, _: &mut Interpreter, _: &[Value]) -> anyhow::Result<Value> {
        let utc = Utc::now().naive_utc();
        let tz = chrono_tz::Australia::Melbourne
            .from_local_datetime(&utc)
            .unwrap();

        Ok(Value::String(tz.to_string()))
    }

    fn arity(&self) -> u8 {
        0
    }

    fn name(&self) -> Rc<String> {
        self.name.clone()
    }
}
