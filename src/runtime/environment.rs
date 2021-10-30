use std::{
    cell::RefCell,
    collections::{hash_map::Entry, HashMap},
    rc::Rc,
};

use ahash::RandomState;

use super::Value;

type ValuesMap = HashMap<Rc<String>, Value, RandomState>;
// type ValuesMap = HashMap<Rc<String>, Value>;

#[derive(Clone, Debug, PartialEq)]
pub struct Environment {
    pub inner: Inner,
}

impl Environment {
    pub fn ancestor(
        root: &Rc<RefCell<Environment>>,
        hops: usize,
    ) -> Option<Rc<RefCell<Environment>>> {
        if hops == 0 {
            Some(root.clone())
        } else {
            match &root.borrow().inner.enclosing {
                None => None,
                Some(enclosing) => Environment::ancestor(enclosing, hops - 1),
            }
        }
    }
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

    pub fn get(&self, key: &String) -> Option<Value> {
        self.inner.get(key)
    }

    pub fn assign(&mut self, key: Rc<String>, val: Value) -> bool {
        self.inner.assign(key, val)
    }

    pub fn define(&mut self, key: Rc<String>, value: Value) {
        self.inner.define(key, value);
    }

    pub fn clone_values(&self) -> ValuesMap {
        self.inner.values.clone()
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct Inner {
    enclosing: Option<Rc<RefCell<Environment>>>,
    pub values: ValuesMap,
}

impl Inner {
    fn new() -> Self {
        Self {
            enclosing: None,
            values: HashMap::default(),
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
            values: HashMap::default(),
        }
    }
}
