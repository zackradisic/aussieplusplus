use std::fmt::Display;

use super::{Ident, Stmt};

#[derive(Clone, Debug, PartialEq)]
pub struct FnDecl {
    pub ident: Ident,
    pub params: Vec<Ident>,
    pub body: Vec<Stmt>,
}

impl FnDecl {
    pub fn new(ident: Ident, params: Vec<Ident>, body: Vec<Stmt>) -> Self {
        Self {
            ident,
            params,
            body,
        }
    }

    pub fn name(&self) -> String {
        self.ident.name()
    }
}

impl Display for FnDecl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}({})",
            self.ident,
            self.params
                .iter()
                .map(|ident| ident.to_string())
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}
