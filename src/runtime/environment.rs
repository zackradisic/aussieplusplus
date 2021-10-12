use std::{
    cell::RefCell,
    collections::{hash_map::Entry, HashMap},
    rc::Rc,
};

use super::Value;

#[derive(Clone, Debug, PartialEq)]
pub struct Environment {
    pub inner: Inner,
}

impl Default for Environment {
    fn default() -> Self {
        Self {
            inner: Inner::new(),
        }
    }
}

impl Environment {
    pub fn new_with_enclosing(inner: Rc<RefCell<Environment>>) -> Self {
        Self {
            inner: Inner::with_enclosing(inner),
        }
    }

    pub fn print_values(&self) {
        // println!("values: {:?}", self.values);
    }

    pub fn get(&self, key: &String) -> Option<Value> {
        self.inner.get(key)
    }

    pub fn assign(&mut self, key: Rc<String>, val: Value) -> bool {
        self.inner.assign(key, val)
    }

    pub fn define(&mut self, key: Rc<String>, value: Value) {
        self.inner.define(key, value);
    }

    pub fn clone_values(&self) -> HashMap<Rc<String>, Value> {
        self.inner.values.clone()
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct Inner {
    enclosing: Option<Rc<RefCell<Environment>>>,
    pub values: HashMap<Rc<String>, Value>,
}

impl Inner {
    fn new() -> Self {
        Self {
            enclosing: None,
            values: HashMap::new(),
        }
    }

    fn define(&mut self, name: Rc<String>, value: Value) {
        self.values.insert(name, value);
    }

    fn get(&self, name: &String) -> Option<Value> {
        match self.values.get(name) {
            None => match &self.enclosing {
                None => None,
                Some(enclosing) => enclosing.borrow().get(name),
            },
            Some(val) => Some(val.clone()),
        }
    }

    fn assign(&mut self, name: Rc<String>, value: Value) -> bool {
        match self.values.entry(name.clone()) {
            Entry::Vacant(_) => match &mut self.enclosing {
                None => false,
                Some(enclosing) => enclosing.borrow_mut().assign(name, value),
            },
            Entry::Occupied(mut entry) => {
                entry.insert(value);
                true
            }
        }
    }

    fn with_enclosing(enclosing: Rc<RefCell<Environment>>) -> Self {
        Self {
            enclosing: Some(enclosing),
            values: HashMap::new(),
        }
    }
}
