use std::cell::RefCell;
use std::fmt::Display;
use std::rc::Rc;
use std::{thread, time::Duration};

#[cfg(not(target_os = "emscripten"))]
use chrono::offset::TimeZone;
#[cfg(not(target_os = "emscripten"))]
use chrono::Utc;

use rand::prelude::ThreadRng;
use rand::Rng;

use crate::runtime::error::RuntimeError;
use crate::runtime::{Interpreter, Value};

use super::AussieCallable;

#[cfg(target_os = "emscripten")]
use std::os::raw::c_char;
#[cfg(target_os = "emscripten")]
extern "C" {
    pub fn aussie_time() -> *mut c_char;
}
#[derive(Clone, PartialEq, Debug)]
pub enum BuiltIn {
    Sleep(Sleep),
    Time(Time),
    Rand(Rand),
}

impl BuiltIn {
    pub fn lookup(name: &str) -> Option<Self> {
        match name {
            "HitTheSack" => Some(BuiltIn::Sleep(Sleep::default())),
            "GimmeTime" => Some(BuiltIn::Time(Time::default())),
            "ChuckSomeDice" => Some(BuiltIn::Rand(Rand::default())),
            _ => None,
        }
    }
}

impl AussieCallable for BuiltIn {
    fn call(&self, interpreter: &mut Interpreter, args: &[Value]) -> anyhow::Result<Value> {
        match self {
            Self::Sleep(sleep) => sleep.call(interpreter, args),
            Self::Time(time) => time.call(interpreter, args),
            Self::Rand(rand) => rand.call(interpreter, args),
        }
    }

    fn arity(&self) -> u8 {
        match self {
            Self::Sleep(sleep) => sleep.arity(),
            Self::Time(time) => time.arity(),
            Self::Rand(rand) => rand.arity(),
        }
    }

    fn name(&self) -> &Rc<str> {
        match self {
            Self::Sleep(sleep) => sleep.name(),
            Self::Time(time) => time.name(),
            Self::Rand(rand) => rand.name(),
        }
    }
}

impl Display for BuiltIn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Sleep(s) => write!(f, "{}(ms)", s.name()),
            Self::Time(t) => write!(f, "{}()", t.name()),
            Self::Rand(r) => write!(f, "{}(start, end)", r.name()),
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct Sleep {
    name: Rc<str>,
}

impl Default for Sleep {
    fn default() -> Self {
        Self {
            name: Rc::from("HitTheSack"),
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

    fn name(&self) -> &Rc<str> {
        &self.name
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct Time {
    name: Rc<str>,
}

impl Default for Time {
    fn default() -> Self {
        Self {
            name: Rc::from("GimmeTime"),
        }
    }
}

impl AussieCallable for Time {
    #[cfg(not(target_os = "emscripten"))]
    fn call(&self, _: &mut Interpreter, _: &[Value]) -> anyhow::Result<Value> {
        let utc = Utc::now().naive_utc();
        let tz = chrono_tz::Australia::Melbourne.from_utc_datetime(&utc);

        Ok(Value::String(tz.to_string()))
    }

    #[cfg(target_os = "emscripten")]
    fn call(&self, _: &mut Interpreter, _: &[Value]) -> anyhow::Result<Value> {
        use std::ffi::CString;

        let str = unsafe {
            CString::from_raw(aussie_time())
                .to_str()
                .unwrap()
                .to_string()
        };

        Ok(Value::String(str))
    }

    fn arity(&self) -> u8 {
        0
    }

    fn name(&self) -> &Rc<str> {
        &self.name
    }
}

#[derive(Clone, Debug)]
pub struct Rand {
    name: Rc<str>,
    rng: RefCell<ThreadRng>,
}

impl PartialEq for Rand {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Default for Rand {
    fn default() -> Self {
        Self {
            name: Rc::from("ChuckSomeDice"),
            rng: RefCell::new(rand::thread_rng()),
        }
    }
}

impl AussieCallable for Rand {
    fn call(&self, _: &mut Interpreter, args: &[Value]) -> anyhow::Result<Value> {
        let (start, end) = match (&args[0], &args[1]) {
            (Value::Number(a), Value::Number(b)) => (*a as i64, *b as i64),
            _ => {
                return Err(RuntimeError::General(
                    "OI MATE, CAN YA FUCKIN' COUNT?? EXPECTED A NUMBER".into(),
                )
                .into())
            }
        };

        if start >= end {
            return Err(RuntimeError::General(
                "OI MATE, CAN YA FUCKIN' COUNT?? START MUST BE LESS THAN END!!".into(),
            )
            .into());
        }

        Ok(Value::Number(
            self.rng.borrow_mut().gen_range(start..end) as f64
        ))
    }

    fn arity(&self) -> u8 {
        2
    }

    fn name(&self) -> &Rc<str> {
        &self.name
    }
}
