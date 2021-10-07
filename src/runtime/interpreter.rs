use anyhow::Result;
use std::{
    cell::RefCell,
    fmt::Arguments,
    io::{stdout, Write},
    ops::Add,
    rc::Rc,
};

use crate::ast::{BinaryOp, Expr, ExprNode, Match, Pattern, Stmt, UnaryOp};

use super::{
    environment::Environment,
    error::RuntimeError,
    exit::{Exit, ExitKind},
    RuntimePartialEq, Value,
};

pub struct Interpreter<'a> {
    writer: Option<&'a mut dyn Write>,
    env: Rc<RefCell<Environment>>,
}

impl<'a> Interpreter<'a> {
    pub fn new() -> Self {
        Self {
            writer: None,
            env: Rc::new(RefCell::new(Environment::new())),
        }
    }

    pub fn new_with_writer(writer: &'a mut dyn Write) -> Interpreter<'a> {
        Interpreter {
            writer: Some(writer),
            env: Rc::new(RefCell::new(Environment::new())),
        }
    }

    pub fn env(&self) -> Rc<RefCell<Environment>> {
        self.env.clone()
    }

    fn print(&mut self, args: Arguments<'_>) {
        use std::borrow::BorrowMut;
        let w = self.writer.borrow_mut();
        if let Some(w) = w {
            let _ = write!(w, "{}\n", args).unwrap();
        }
        stdout().write_fmt(args).unwrap();
    }

    pub fn interpret(&mut self, stmts: Vec<Stmt>) -> Result<()> {
        for stmt in stmts {
            match self.execute_stmt(&stmt)? {
                None => {}
                Some(_) => return Ok(()),
            };
        }
        Ok(())
    }

    fn execute_stmt(&mut self, stmt: &Stmt) -> Result<Exit> {
        match stmt {
            Stmt::Print(expr) => {
                let val = self.evaluate(expr)?;
                self.print(format_args!("{}", val));
                Ok(None)
            }
            Stmt::Match(Match {
                val,
                branches,
                default,
            }) => {
                let val = self.evaluate(val)?;

                for branch in branches {
                    if branch.pat.runtime_eq(&val) {
                        return self.execute_block(
                            &branch.body,
                            Environment::new_with_enclosing(Some(self.env())),
                        );
                    }
                }

                match default {
                    Some(branch) if !branch.pat.runtime_eq(&val) => {
                        let mut env = Environment::new_with_enclosing(Some(self.env()));
                        let var = if let Pattern::Var(v) = &branch.pat {
                            Some(v)
                        } else {
                            None
                        };
                        env.define(var.unwrap().name(), val);

                        self.execute_block(&branch.body, env)
                    }
                    _ => Ok(None),
                }
            }
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
                        Some(ExitKind::Break) => return Ok(None),
                        Some(ExitKind::Return) => return Ok(Some(ExitKind::Return)),
                    };
                }

                Ok(None)
            }
            Stmt::Block(stmts) => {
                self.execute_block(stmts, Environment::new_with_enclosing(Some(self.env())))
            }
        }
    }

    fn execute_block(&mut self, stmts: &Vec<Stmt>, env: Environment) -> Result<Exit> {
        let previous = self.env.clone();
        self.env = Rc::new(RefCell::new(env));

        for stmt in stmts {
            match self.execute_stmt(stmt)? {
                None => {}
                Some(s) => return Ok(Some(s)),
            };
        }

        self.env = previous;

        Ok(None)
    }
}

impl<'a> Interpreter<'a> {
    pub fn evaluate(&mut self, node: &ExprNode) -> Result<Value> {
        match node.expr() {
            // Expr::Call(ref expr_callee, ref token, ref args) => {
            //     let callee = self.evaluate(expr_callee)?;

            //     if !callee.is_callable() {
            //         return Err(RuntimeError::InvalidCallee(token.line).into());
            //     }

            //     let callable = match callee {
            //         Value::Callable(RloxCallable::Function(Function::UserDefined(func))) => func,
            //         _ => todo!(),
            //     };

            //     if args.len() != callable.arity().into() {
            //         return Err(RuntimeError::InvalidArity(
            //             token.line,
            //             callable.arity(),
            //             args.len(),
            //         )
            //         .into());
            //     }

            //     callable.call(self, args)
            // }
            // Expr::Logical(left, op, right) => match op {
            //     LogicalOp::And => match Self::is_truthy(&self.evaluate(left)?) {
            //         true => {
            //             let right = self.evaluate(right)?;
            //             if Self::is_truthy(&right) {
            //                 return Ok(right);
            //             }
            //             Ok(Value::Bool(false))
            //         }
            //         false => Ok(Value::Bool(false)),
            //     },
            //     LogicalOp::Or => {
            //         let left = self.evaluate(left)?;
            //         match Self::is_truthy(&left) {
            //             true => Ok(left),
            //             false => {
            //                 let right = self.evaluate(right)?;
            //                 if Self::is_truthy(&right) {
            //                     return Ok(right);
            //                 }
            //                 Ok(Value::Bool(false))
            //             }
            //         }
            //     }
            // },
            // Expr::Assign(ref var, ref expr) => {
            //     let value = self.evaluate(expr)?;
            //     self.env
            //         .borrow_mut()
            //         .assign(var.identifier.name.clone(), value.clone());
            //     Ok(value)
            // }
            Expr::Var(ref var) => Ok(self
                .env
                .borrow()
                .get(&var.ident().name())
                .map_or_else(|| Value::Nil, |v| v)),
            Expr::Literal(ref val) => Ok(val.clone()),
            Expr::Grouping(ref expr) => self.evaluate(expr),
            Expr::Unary(op, ref expr) => {
                let right = self.evaluate(expr)?;

                match op {
                    UnaryOp::Bang => Ok(Value::Bool(!Self::is_truthy(&right))),
                }
            }
            Expr::Binary(ref left_expr, op, ref right_expr) => {
                let line = left_expr.line();
                let a = self.evaluate(&left_expr)?;
                let b = self.evaluate(&right_expr)?;

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
    }
}

impl<'a> Interpreter<'a> {
    fn is_truthy(val: &Value) -> bool {
        match val {
            Value::Bool(b) => b.clone(),
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
            (Value::Number(a), Value::Number(b)) => a == b,
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
