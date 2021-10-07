use std::{
    cell::RefCell,
    collections::{hash_map::Entry, HashMap},
    rc::Rc,
};

use super::Value;

pub struct Environment {
    values: HashMap<String, Value>,
    enclosing: Option<Rc<RefCell<Environment>>>,
}

impl Environment {
    pub fn new() -> Self {
        Self::new_with_enclosing(None)
    }

    pub fn new_with_enclosing(enclosing: Option<Rc<RefCell<Environment>>>) -> Self {
        let mut env = Self {
            enclosing,
            values: HashMap::new(),
        };

        // env.define(
        //     "clock".to_string(),
        //     Value::Callable(RloxCallable::Function(Function::BuiltIn(BuiltIn::Clock(
        //         Clock {},
        //     )))),
        // );

        env
    }

    pub fn define(&mut self, name: String, value: Value) {
        self.values.insert(name, value);
    }

    pub fn get(&self, name: &String) -> Option<Value> {
        match self.values.get(name).cloned() {
            None => match &self.enclosing {
                None => None,
                Some(enclosing) => enclosing.borrow().get(name),
            },
            Some(val) => Some(val),
        }
    }

    pub fn assign(&mut self, name: String, value: Value) -> bool {
        match self.values.entry(name.clone()) {
            Entry::Vacant(_) => match self.enclosing.clone() {
                None => false,
                Some(enclosing) => (*enclosing).borrow_mut().assign(name, value),
            },
            Entry::Occupied(mut entry) => {
                entry.insert(value);
                true
            }
        }
    }
}
