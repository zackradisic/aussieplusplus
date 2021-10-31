use std::{fmt::Display, rc::Rc};

#[derive(Clone, Debug, PartialEq)]
pub struct Var {
    pub ident: Ident,
    // is defined. 0 means it belongs in the current environment
    pub scope_distance: usize,
}

impl Var {
    pub fn new(ident: Ident, scope_distance: usize) -> Self {
        Self {
            ident,
            scope_distance,
        }
    }

    pub fn ident(&self) -> Ident {
        self.ident.clone()
    }

    pub fn name(&self) -> &Rc<str> {
        &self.ident.name
    }

    pub fn line(&self) -> usize {
        self.ident.line
    }
}

impl From<(Ident, usize)> for Var {
    fn from(val: (Ident, usize)) -> Self {
        Self {
            ident: val.0,
            scope_distance: val.1,
        }
    }
}

impl From<(String, usize, usize)> for Var {
    fn from(tup: (String, usize, usize)) -> Self {
        Self {
            ident: (tup.0, tup.1).into(),
            scope_distance: tup.2,
        }
    }
}

impl From<(&str, usize, usize)> for Var {
    fn from(tup: (&str, usize, usize)) -> Self {
        (tup.0.to_string(), tup.1, tup.2).into()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Ident {
    pub name: Rc<str>,
    line: usize,
}

impl Ident {
    pub fn new(name: String, line: usize) -> Self {
        Self {
            name: Rc::from(name),
            line,
        }
    }

    pub fn line(&self) -> usize {
        self.line
    }
}

impl From<(String, usize)> for Ident {
    fn from(tup: (String, usize)) -> Self {
        Self {
            name: Rc::from(tup.0),
            line: tup.1,
        }
    }
}

impl From<(&str, usize)> for Ident {
    fn from(tup: (&str, usize)) -> Self {
        (tup.0.to_string(), tup.1).into()
    }
}

impl Display for Ident {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}
