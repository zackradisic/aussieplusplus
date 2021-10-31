use std::{cell::RefCell, fmt::Display, rc::Rc};

use anyhow::Result;

use crate::{
    ast::FnDecl,
    runtime::{exit::ExitKind, Environment, Interpreter, Value},
};

use super::{AussieCallable, BuiltIn};

#[derive(Clone, PartialEq, Debug)]
pub enum Function {
    UserDefined(Box<UserDefined>),
    BuiltIn(BuiltIn),
}

impl AussieCallable for Function {
    fn call(&self, interpreter: &mut Interpreter, args: &[Value]) -> Result<Value> {
        match self {
            Function::UserDefined(func) => func.call(interpreter, args),
            Function::BuiltIn(built_in) => built_in.call(interpreter, args),
        }
    }

    fn arity(&self) -> u8 {
        match self {
            Function::UserDefined(func) => func.arity(),
            Function::BuiltIn(built_in) => built_in.arity(),
        }
    }

    fn name(&self) -> &Rc<str> {
        match self {
            Function::UserDefined(func) => func.name(),
            Function::BuiltIn(func) => func.name(),
        }
    }
}

impl Display for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UserDefined(func) => write!(f, "{}", func.to_string()),
            Self::BuiltIn(b) => write!(f, "{}", b.to_string()),
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct UserDefined {
    decl: FnDecl,
    // The closure this function was defined in
    env: Rc<RefCell<Environment>>,
}

impl UserDefined {
    pub fn new(decl: FnDecl, env: Rc<RefCell<Environment>>) -> Self {
        Self { decl, env }
    }
}

impl AussieCallable for UserDefined {
    fn call(&self, interpreter: &mut Interpreter, args: &[Value]) -> Result<Value> {
        let mut env = Environment::new_with_enclosing(self.env.clone());

        for (parameter, value) in self.decl.params.iter().zip(args.iter()) {
            env.define(parameter.name.clone(), value.clone());
        }

        if let Some(ExitKind::Return(val)) = interpreter.execute_block(
            &self.decl.body,
            Rc::new(RefCell::new(Environment::new_with_enclosing(Rc::new(
                RefCell::new(env),
            )))),
        )? {
            return Ok(val);
        }

        Ok(Value::Nil)
    }

    fn arity(&self) -> u8 {
        self.decl.params.len() as u8
    }

    fn name(&self) -> &Rc<str> {
        self.decl.name()
    }
}

impl Display for UserDefined {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.decl.to_string())
    }
}
