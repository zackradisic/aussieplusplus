use std::{collections::HashMap, mem, rc::Rc};

use itertools::Itertools;

use crate::{
    ast::{Expr, ExprNode, FnDecl, ForLoop, Ident, If, Match, Pattern, Stmt, Var as AstVar},
    token::Token,
};

macro_rules! with_scope {
    ($self:ident, $code:tt) => {
        $self.begin_scope();
        $code;
        $self.end_scope();
    };
}

enum FunctionKind {
    None,
    Function,
}

struct Var {
    // To prevent reading a variable in its initializer
    in_initializer: bool,
    immutable: bool,
}

pub struct Resolver {
    scopes: Vec<HashMap<Rc<String>, Var>>,
    had_error: bool,
    cur_fn: FunctionKind,
}

impl Resolver {
    pub fn new() -> Self {
        Self {
            scopes: vec![HashMap::new()],
            had_error: false,
            cur_fn: FunctionKind::None,
        }
    }

    pub fn resolve(&mut self, stmts: &mut [Stmt]) -> bool {
        for stmt in stmts {
            self.stmt(stmt);
        }

        self.had_error
    }

    fn stmt(&mut self, stmt: &mut Stmt) {
        match stmt {
            Stmt::Block(stmts) => self.block_stmt(stmts),
            Stmt::VarDecl(name, init) => self.var_stmt(name, init),
            Stmt::FnDecl(decl) => self.func_stmt(decl, FunctionKind::Function),
            Stmt::If(If { cond, then, else_ }) => self.if_stmt(cond, then, else_),
            Stmt::Print(expr) => self.print_stmt(expr),
            Stmt::Return(tok, expr) => self.ret_stmt(tok, expr),
            Stmt::While(node) => self.while_stmt(&mut node.cond, &mut node.body),
            Stmt::For(for_loop) => self.for_stmt(for_loop),
            Stmt::Match(match_) => self.match_stmt(match_),
            Stmt::Import(ident) => self.import_stmt(ident),
            Stmt::Break(_) => {}
            Stmt::Exit(_) => {}
            Stmt::Expr(expr) => self.expr(expr.expr_mut()),
        }
    }

    fn block_stmt(&mut self, stmts: &mut [Stmt]) {
        self.begin_scope();
        self.resolve(stmts);
        self.end_scope();
    }

    fn var_stmt(&mut self, name: &Ident, init: &mut Option<ExprNode>) {
        self.declare(name, false);
        if let Some(init) = init {
            self.expr(init.expr_mut());
        }
        self.define(name);
    }

    fn func_stmt(&mut self, decl: &mut FnDecl, kind: FunctionKind) {
        self.declare(&decl.ident, true);
        self.define(&decl.ident);

        self.resolve_fn(decl, kind)
    }

    fn if_stmt(&mut self, cond: &mut ExprNode, then: &mut Stmt, elze: &mut Option<Box<Stmt>>) {
        self.expr(cond.expr_mut());
        self.stmt(then);

        if let Some(e) = elze {
            self.stmt(e);
        }
    }

    fn print_stmt(&mut self, expr: &mut ExprNode) {
        self.expr(expr.expr_mut());
    }

    fn ret_stmt(&mut self, tok: &mut Token, expr: &mut Option<ExprNode>) {
        if let FunctionKind::None = self.cur_fn {
            self.print_error(
                tok.line(),
                &tok.kind.to_string(),
                "YA CAN ONLY RETURN IN FUNCTIONS DUMMY!",
            );
        }
        if let Some(expr) = expr {
            self.expr(expr.expr_mut());
        }
    }

    fn while_stmt(&mut self, cond: &mut ExprNode, body: &mut Vec<Stmt>) {
        self.expr(cond.expr_mut());
        body.iter_mut().for_each(|stmt| self.stmt(stmt));
    }

    fn for_stmt(&mut self, for_loop: &mut ForLoop) {
        self.expr(for_loop.range.0.expr_mut().expr_mut());
        self.expr(for_loop.range.1.expr_mut().expr_mut());

        with_scope!(self, {
            self.declare(&for_loop.var.ident, false);
            self.define(&for_loop.var.ident());

            for_loop.body.iter_mut().for_each(|stmt| {
                self.stmt(stmt);
            });
        });
    }

    fn match_stmt(&mut self, match_: &mut Match) {
        self.expr(match_.val.expr_mut());

        match_.branches.iter_mut().for_each(|branch| {
            self.block_stmt(&mut branch.body);
        });

        if let Some(default) = &mut match_.default {
            if let Pattern::Var(var) = &mut default.pat {
                with_scope!(self, {
                    self.declare(&var.ident, false);
                    self.define(&var.ident());
                    default.body.iter_mut().for_each(|stmt| {
                        self.stmt(stmt);
                    });
                });
            }
        }
    }

    fn import_stmt(&mut self, name: &Ident) {
        self.declare(name, true);
        self.define(name);
    }

    fn declare(&mut self, name: &Ident, immutable: bool) {
        let mut exists = false;
        if let Some(scope) = self.scopes.last_mut() {
            let name = name.name();
            if scope.contains_key(&*name) {
                exists = true;
            }
            scope.insert(
                name,
                Var {
                    in_initializer: false,
                    immutable,
                },
            );
        }

        if exists {
            self.print_error(
                name.line(),
                &name.name(),
                "WAKE UP FUCK-WIT! A VARIABLE WITH THAT NAME ALREADY EXISTS IN THIS SCOPE.",
            )
        }
    }

    fn define(&mut self, name: &Ident) {
        if let Some(scope) = self.scopes.last_mut() {
            if let Some(v) = scope.get_mut(&name.name()) {
                v.in_initializer = true;
            } else {
                self.print_error(name.line(), &name.name(), "CAN'T DEFINE AN UNDECLARED VAR")
            }
        }
    }

    fn begin_scope(&mut self) {
        self.scopes.push(HashMap::new())
    }

    fn end_scope(&mut self) {
        self.scopes.pop();
    }

    fn resolve_local(&mut self, var: &mut AstVar) -> &mut Var {
        for (i, scope) in self.scopes.iter_mut().rev().enumerate() {
            if let Some(v) = scope.get_mut(&*var.name()) {
                var.scope_distance = i;
                return v;
            }
        }

        panic!("Unable to resolve var: {:?}", var)
    }

    fn resolve_fn(&mut self, decl: &mut FnDecl, kind: FunctionKind) {
        let enclosing_fn = mem::replace(&mut self.cur_fn, kind);

        self.begin_scope();
        decl.params.iter().for_each(|param| {
            self.declare(param, false);
            self.define(param);
        });
        self.block_stmt(&mut decl.body);
        self.end_scope();

        self.cur_fn = enclosing_fn;
    }
}

impl Default for Resolver {
    fn default() -> Self {
        Self::new()
    }
}

impl Resolver {
    fn expr(&mut self, expr: &mut Expr) {
        match expr {
            Expr::Var(v) => self.expr_var(v),
            Expr::Assign(a, init) => self.expr_assign(a, init),
            Expr::Binary(left, _, right) => {
                self.expr(left.expr_mut());
                self.expr(right.expr_mut());
            }
            Expr::Call(callee, _, args) => {
                self.expr(callee.expr_mut());
                args.iter_mut().for_each(|arg| self.expr(arg.expr_mut()));
            }
            Expr::Grouping(expr) => self.expr(expr.expr_mut()),
            Expr::Literal(_) => {}
            Expr::Logical(left, _, right) => {
                self.expr(left.expr_mut());
                self.expr(right.expr_mut());
            }
            Expr::Unary(_, expr) => self.expr(expr.expr_mut()),
        }
    }

    fn expr_var(&mut self, var: &mut AstVar) {
        let name = var.name();
        if let Some(scope) = self.scopes.last() {
            match scope.get(&name) {
                Some(Var {
                    in_initializer: initialized,
                    ..
                }) if !initialized => {
                    return self.print_error(
                    var.line(),
                        &name,
                        "FUCK ME DEAD MATE... YOU JUST TRIED TO READ A VARIABLE IN ITS INITIALIZER!",
                );
                }
                _ => {}
            };
        }

        self.resolve_local(var);
    }

    fn expr_assign(&mut self, var: &mut AstVar, init: &mut ExprNode) {
        self.expr(init.expr_mut());
        let v = self.resolve_local(var);
        if v.immutable {
            self.print_error(var.line(), &var.name(), "OI, YA CAN'T REDEFINE THIS!")
        }
    }
}

impl Resolver {
    fn print_error(&mut self, line: usize, name: &str, msg: &str) {
        self.had_error = true;
        eprintln!("[line {}] {}: {}", line, name, msg)
    }
}
