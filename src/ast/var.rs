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

    pub fn name(&self) -> String {
        self.ident.name()
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
    name: String,
    line: usize,
}

impl Ident {
    pub fn new(name: String, line: usize) -> Self {
        Self { name, line }
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn line(&self) -> usize {
        self.line
    }
}

impl From<(String, usize)> for Ident {
    fn from(tup: (String, usize)) -> Self {
        Self {
            name: tup.0,
            line: tup.1,
        }
    }
}

impl From<(&str, usize)> for Ident {
    fn from(tup: (&str, usize)) -> Self {
        (tup.0.to_string(), tup.1).into()
    }
}
