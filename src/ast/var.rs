use std::{fmt::Display, rc::Rc};

#[derive(Clone, Debug, PartialEq)]
pub struct Var {
    ident: Ident,
}

impl Var {
    pub fn new(ident: Ident) -> Self {
        Self { ident }
    }

    pub fn ident(&self) -> Ident {
        self.ident.clone()
    }

    pub fn name(&self) -> Rc<String> {
        self.ident.name()
    }

    pub fn line(&self) -> usize {
        self.ident.line
    }
}

impl From<Ident> for Var {
    fn from(ident: Ident) -> Self {
        Self { ident }
    }
}

impl From<(String, usize)> for Var {
    fn from(tup: (String, usize)) -> Self {
        Self { ident: tup.into() }
    }
}

impl From<(&str, usize)> for Var {
    fn from(tup: (&str, usize)) -> Self {
        (tup.0.to_string(), tup.1).into()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Ident {
    name: Rc<String>,
    line: usize,
}

impl Ident {
    pub fn new(name: String, line: usize) -> Self {
        Self {
            name: Rc::new(name),
            line,
        }
    }

    pub fn name(&self) -> Rc<String> {
        self.name.clone()
    }

    pub fn line(&self) -> usize {
        self.line
    }
}

impl From<(String, usize)> for Ident {
    fn from(tup: (String, usize)) -> Self {
        Self {
            name: Rc::new(tup.0),
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
