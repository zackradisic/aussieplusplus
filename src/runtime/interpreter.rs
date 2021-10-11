use anyhow::Result;
use std::{
    cell::RefCell,
    fmt::Arguments,
    io::{stdout, Write},
    mem,
    ops::Add,
    rc::Rc,
};

use crate::{
    ast::{BinaryOp, Expr, ExprNode, ForLoop, LogicalOp, Match, Pattern, Range, Stmt, UnaryOp},
    parser::error::ParseError,
    runtime::AussieCallable,
    token::Token,
};

use super::{
    environment::Environment,
    error::RuntimeError,
    exit::{Exit, ExitKind},
    Callable, Function, RuntimePartialEq, UserDefined, Value,
};

pub struct Interpreter<'a> {
    writer: Option<&'a mut dyn Write>,
    env: Rc<RefCell<Environment>>,
}

impl<'a> Default for Interpreter<'a> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> Interpreter<'a> {
    pub fn new() -> Self {
        Self {
            writer: None,
            env: Rc::new(RefCell::new(Environment::default())),
        }
    }

    pub fn new_with_writer(writer: &'a mut dyn Write) -> Interpreter<'a> {
        Interpreter {
            writer: Some(writer),
            env: Rc::new(RefCell::new(Environment::default())),
        }
    }

    pub fn env(&self) -> Rc<RefCell<Environment>> {
        self.env.clone()
    }

    fn print(&mut self, args: Arguments<'_>) {
        use std::borrow::BorrowMut;
        let w = self.writer.borrow_mut();
        if let Some(w) = w {
            let _ = writeln!(w, "{}", args).unwrap();
        }
        println!("{}", args);
        stdout().flush().unwrap();
    }

    pub fn interpret(&mut self, stmts: Vec<Stmt>) -> Result<()> {
        for stmt in stmts {
            match self.execute_stmt(&stmt)? {
                None => {}
                Some(ExitKind::Break(line)) => return Err(RuntimeError::InvalidBreak(line).into()),
                Some(_) => return Ok(()),
            };
        }
        Ok(())
    }

    fn execute_stmt(&mut self, stmt: &Stmt) -> Result<Exit> {
        match stmt {
            Stmt::Return(expr) => match expr {
                None => Ok(Some(ExitKind::Return(Value::Nil))),
                Some(val) => Ok(Some(ExitKind::Return(self.evaluate(val)?))),
            },
            Stmt::FnDecl(fn_decl) => {
                let function =
                    Callable::Function(Function::UserDefined(Box::new(UserDefined::new(
                        fn_decl.clone(),
                        Rc::new(RefCell::new(Environment::new_with_enclosing(
                            self.env.clone(),
                        ))),
                    ))));

                self.env
                    .borrow_mut()
                    .define(fn_decl.name(), Value::Callable(Rc::new(function)));

                Ok(None)
            }
            Stmt::Break(tok) => Ok(Some(ExitKind::Break(tok.line()))),
            Stmt::While(_while_loop) => {
                todo!();
                // let mut env = Environment::new_with_enclosing(Some(self.env()));
                // let name = while_loop.var.name();
                // env.define(name.clone(), value)
            }
            Stmt::For(for_loop) => self.execute_for_loop(for_loop),
            Stmt::Print(expr) => {
                let val = self.evaluate(expr)?;
                self.print(format_args!("{}", val));
                Ok(None)
            }
            Stmt::Match(m) => self.execute_match(m),
            Stmt::Expr(expr_node) => {
                let _ = self.evaluate(expr_node)?;
                Ok(None)
            }
            Stmt::VarDecl(ident, initializer) => {
                let value = match initializer {
                    None => Value::Nil,
                    Some(expr_node) => self.evaluate(expr_node)?,
                };

                self.env.borrow_mut().define(ident.name(), value);

                Ok(None)
            }
            Stmt::If(cond, then) => {
                let val = self.evaluate(cond)?;

                if Self::is_truthy(&val) {
                    match self.execute_stmt(then)? {
                        None => {}
                        Some(exit) => return Ok(Some(exit)),
                    };
                }

                Ok(None)
            }
            Stmt::Block(stmts) => self.execute_block(
                stmts,
                Rc::new(RefCell::new(Environment::new_with_enclosing(self.env()))),
            ),
        }
    }

    fn execute_match(&mut self, m: &Match) -> Result<Exit> {
        let Match {
            val,
            branches,
            default,
        } = m;
        let val = self.evaluate(val)?;

        for branch in branches {
            if branch.pat.runtime_eq(&val) {
                return self.execute_block(
                    &branch.body,
                    Rc::new(RefCell::new(Environment::new_with_enclosing(self.env()))),
                );
            }
        }

        match default {
            Some(branch) if !branch.pat.runtime_eq(&val) => {
                let mut env = Environment::new_with_enclosing(self.env());
                let var = if let Pattern::Var(v) = &branch.pat {
                    Some(v)
                } else {
                    None
                };
                env.define(var.unwrap().name(), val);

                self.execute_block(&branch.body, Rc::new(RefCell::new(env)))
            }
            _ => Ok(None),
        }
    }

    fn execute_for_loop(&mut self, for_loop: &ForLoop) -> Result<Exit> {
        let mut env = Environment::new_with_enclosing(self.env());
        let start = match self.evaluate(for_loop.range.0.expr())? {
            Value::Number(n) => n,
            other => {
                return Err(ParseError::InvalidRange(
                    for_loop.var.line(),
                    "start".into(),
                    other.into(),
                )
                .into())
            }
        };
        let end = match self.evaluate(for_loop.range.1.expr())? {
            Value::Number(n) => n,
            other => {
                let line = for_loop.var.line();
                return Err(ParseError::InvalidRange(line, "start".into(), other.into()).into());
            }
        };

        let range = (
            for_loop.range.0.to_evaluated(start),
            for_loop.range.1.to_evaluated(end),
        );

        let (mut i, _) = range.values();

        let var_name = for_loop.var.name();
        env.define(var_name.clone(), Value::Number(i));

        let env = Rc::new(RefCell::new(env));

        while range.satisfied(i) {
            match self.execute_block(&for_loop.body, env.clone())? {
                None => {}
                Some(ExitKind::Break(_)) => break,
                Some(ExitKind::Return(line)) => return Ok(Some(ExitKind::Return(line))),
            };
            range.iterate(&mut i);
            env.borrow_mut().assign(var_name.clone(), Value::Number(i));
        }

        Ok(None)
    }

    pub fn execute_block(&mut self, stmts: &[Stmt], env: Rc<RefCell<Environment>>) -> Result<Exit> {
        let previous = mem::replace(&mut self.env, env);

        for stmt in stmts {
            match self.execute_stmt(stmt)? {
                None => {}
                Some(s) => {
                    self.env = previous;
                    return Ok(Some(s));
                }
            };
        }

        self.env = previous;

        Ok(None)
    }
}

impl<'a> Interpreter<'a> {
    pub fn evaluate(&mut self, node: &ExprNode) -> Result<Value> {
        match node.expr() {
            Expr::Call(expr_callee, token, params) => {
                self.evaluate_call(expr_callee, token, params)
            }
            Expr::Assign(ref var, ref expr) => {
                let value = self.evaluate(expr)?;
                self.env.borrow_mut().assign(var.name(), value.clone());
                Ok(value)
            }
            Expr::Var(ref var) => Ok(self
                .env
                .borrow()
                .get(&var.ident().name())
                .map_or_else(|| Value::Nil, |v| v)),
            Expr::Literal(ref val) => Ok(val.clone()),
            Expr::Grouping(ref expr) => self.evaluate(expr),
            Expr::Unary(op, ref expr) => {
                let right = self.evaluate(expr)?;

                match (op, right) {
                    (UnaryOp::Bang, right) => Ok(Value::Bool(!Self::is_truthy(&right))),
                    (UnaryOp::Minus, Value::Number(right)) => Ok(Value::Number(right * -1f64)),
                    _ => {
                        Err(RuntimeError::new_syntax("invalid unary operation", expr.line()).into())
                    }
                }
            }
            Expr::Logical(left, op, right) => match op {
                LogicalOp::And => match Self::is_truthy(&self.evaluate(left)?) {
                    true => {
                        let right = self.evaluate(right)?;
                        if Self::is_truthy(&right) {
                            return Ok(right);
                        }
                        Ok(Value::Bool(false))
                    }
                    false => Ok(Value::Bool(false)),
                },
                LogicalOp::Or => {
                    let left = self.evaluate(left)?;
                    match Self::is_truthy(&left) {
                        true => Ok(left),
                        false => {
                            let right = self.evaluate(right)?;
                            if Self::is_truthy(&right) {
                                return Ok(right);
                            }
                            Ok(Value::Bool(false))
                        }
                    }
                }
            },
            Expr::Binary(ref left_expr, op, ref right_expr) => {
                self.evaluate_binary(left_expr, op, right_expr)
            }
        }
    }

    fn evaluate_call(
        &mut self,
        expr_callee: &ExprNode,
        token: &Token,
        params: &[ExprNode],
    ) -> Result<Value> {
        let callee = self.evaluate(expr_callee)?;

        let callable = match callee {
            Value::Callable(callable) => callable,
            _ => return Err(RuntimeError::InvalidCallee(token.line()).into()),
        };

        if params.len() != callable.arity().into() {
            return Err(
                RuntimeError::InvalidArity(token.line(), callable.arity(), params.len()).into(),
            );
        }

        let mut args: Vec<Value> = Vec::with_capacity(params.len());
        for arg in params {
            args.push(self.evaluate(arg)?);
        }

        callable.call(self, &args)
    }

    fn evaluate_binary(
        &mut self,
        left_expr: &ExprNode,
        op: &BinaryOp,
        right_expr: &ExprNode,
    ) -> Result<Value> {
        let line = left_expr.line();
        let a = self.evaluate(left_expr)?;
        let b = self.evaluate(right_expr)?;

        match op {
            BinaryOp::Plus => match (a, b) {
                (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a + b)),
                (Value::String(a), Value::String(b)) => Ok(Value::String(a + &b)),
                (Value::String(a), b) => Ok(Value::String(a.add(b.to_string().as_str()))),
                _ => Err(RuntimeError::new_syntax(
                    "Both operands cannot be converted to strings",
                    line,
                )
                .into()),
            },
            BinaryOp::Modulo => {
                let (a, b) = self.unwrap_nums(a, b, line)?;
                Ok(Value::Number(a % b))
            }
            BinaryOp::Minus => {
                let (a, b) = self.unwrap_nums(a, b, line)?;
                Ok(Value::Number(a - b))
            }
            BinaryOp::Divide => {
                let (a, b) = self.unwrap_nums(a, b, line)?;
                Ok(Value::Number(a / b))
            }
            BinaryOp::Multiply => {
                let (a, b) = self.unwrap_nums(a, b, line)?;
                Ok(Value::Number(a * b))
            }
            BinaryOp::Greater => {
                let (a, b) = self.unwrap_nums(a, b, line)?;
                Ok(Value::Bool(a > b))
            }
            BinaryOp::GreaterEqual => {
                let (a, b) = self.unwrap_nums(a, b, line)?;
                Ok(Value::Bool(a >= b))
            }
            BinaryOp::Less => {
                let (a, b) = self.unwrap_nums(a, b, line)?;
                Ok(Value::Bool(a < b))
            }
            BinaryOp::LessEqual => {
                let (a, b) = self.unwrap_nums(a, b, line)?;
                Ok(Value::Bool(a <= b))
            }
            BinaryOp::NotEqual => Ok(Value::Bool(!self.is_equal(a, b))),
            BinaryOp::Equal => Ok(Value::Bool(self.is_equal(a, b))),
        }
    }
}

impl<'a> Interpreter<'a> {
    fn is_truthy(val: &Value) -> bool {
        match val {
            Value::Bool(b) => *b,
            Value::Nil => false,
            _ => true,
        }
    }

    fn is_equal(&self, a: Value, b: Value) -> bool {
        if let Value::Nil = a {
            if let Value::Nil = b {
                return true;
            }
            return false;
        }

        match (a, b) {
            (Value::Number(a), Value::Number(b)) => (a - b).abs() < f64::EPSILON,
            (Value::String(a), Value::String(b)) => a == b,
            (Value::Bool(a), Value::Bool(b)) => a == b,
            _ => false,
        }
    }

    fn unwrap_nums(&self, a: Value, b: Value, line: usize) -> Result<(f64, f64)> {
        match (a, b) {
            (Value::Number(a), Value::Number(b)) => Ok((a, b)),
            _ => Err((RuntimeError::new_syntax("Both operands must be numbers.", line)).into()),
        }
    }
}
