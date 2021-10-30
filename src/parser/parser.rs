use anyhow::{anyhow, Result};

use crate::ast::{
    FnDecl, ForLoop, Ident, If, LogicalOp, Match, MatchBranch, Pattern, RangeBound, Stmt, Var,
    WhileLoop,
};
use crate::runtime::{Value, MAX_ARITY};
use crate::{
    ast::{BinaryOp, Expr, ExprNode, UnaryOp},
    token::{Kind, Token},
};

use super::error::ParseError;

/// Advance token if match succeeds
macro_rules! match_toks {
    ($self:ident, $other:pat => $other_result:expr, $($pat:pat => $result:expr),*) => {
        match $self.peek().kind() {
            $(
                $pat => {
                    let _ = $self.advance();
                    $result
                },
            )*
            $other => $other_result
        }
    }
}

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
    consumed_start: bool,

    // To help discriminate boomerangs vs. gt/lt
    inside_block: usize,
    inside_if: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            consumed_start: false,
            current: 0,
            inside_block: 0,
            inside_if: 0,
        }
    }

    pub fn reset(&mut self, tokens: Vec<Token>) {
        self.current = 0;
        self.tokens = tokens;
    }

    pub fn parse(&mut self) -> Result<Vec<Stmt>> {
        let mut stmts: Vec<Stmt> = Vec::new();
        let mut had_error = false;

        if !self.consumed_start {
            match self.consume_program_start() {
                Ok(_) => {
                    self.consumed_start = true;
                }
                Err(e) => {
                    had_error = true;
                    eprintln!("{:?}", e)
                }
            }
        }

        while !self.is_at_end() {
            match self.declaration() {
                Ok(stmt) => {
                    stmts.push(stmt);
                }
                Err(e) => {
                    had_error = true;
                    eprintln!("{:?}", e);
                }
            }
        }

        if had_error {
            Err(anyhow!("failed to parse"))
        } else {
            Ok(stmts)
        }
    }

    fn declaration(&mut self) -> Result<Stmt> {
        match_toks!(self,
            _ => self.statement(),
            Kind::IReckon => self.var_decl_or_loop(),
            Kind::HardYakkaFor => self.fn_decl()
        )
    }

    fn var_decl_or_loop(&mut self) -> Result<Stmt> {
        if self.match_tok(Kind::IllHaveA) {
            self.consume(Kind::Walkabout)?;
            return self.loops(None);
        }

        let ident = self.consume_ident()?;

        match_toks!(self,
            other => {
                let line = self.peek().line();
                Err(
                    ParseError::ExpectedTokens(vec![Kind::Semicolon, Kind::Assign], other, line).into(),
                )
            },
            Kind::Semicolon => {
                Ok(Stmt::VarDecl(ident, None))
            },
            Kind::Assign => {
                let initializer = self.expression()?;
                self.consume(Kind::Semicolon)?;
                Ok(Stmt::VarDecl(ident, Some(initializer)))
            },
            Kind::Isa => {
                self.consume(Kind::Walkabout)?;
                self.loops(Some(ident))
            }
        )
    }

    fn fn_decl(&mut self) -> Result<Stmt> {
        let name = self.consume_ident()?;
        self.consume(Kind::Is)?;
        let mut params: Vec<Ident> = Vec::new();

        self.consume(Kind::LeftParen)?;
        if !self.match_tok(Kind::RightParen) {
            loop {
                params.push(self.consume_ident()?);

                if !self.match_tok(Kind::Comma) {
                    break;
                }
            }
            self.consume(Kind::RightParen)?;
        }

        self.consume(Kind::LeftBoomerang)?;
        self.inside_block += 1;

        let body = match self.block_statement()? {
            Stmt::Block(stmts) => stmts,
            _ => {
                self.inside_block -= 1;
                return Err(ParseError::Any(
                    name.line(),
                    "STREWTH! EXPECTED BLOCK AFTER FUNCTION DECLARATION".into(),
                )
                .into());
            }
        };
        self.inside_block -= 1;

        Ok(Stmt::FnDecl(FnDecl::new(name, params, body)))
    }

    fn loops(&mut self, ident: Option<Ident>) -> Result<Stmt> {
        match_toks!(self,
            _other => {
                todo!()
            },
            Kind::From => {
                let ident = ident.unwrap();
                let start = match_toks!(self,
                    k =>
                    return Err(ParseError::ExpectedTokens(
                        vec![Kind::LeftParen, Kind::LeftBracket],
                        k,
                        ident.line(),
                    ).into()),
                    Kind::LeftParen => {
                        RangeBound::Exclusive(self.expression()?)
                    },
                    Kind::LeftBracket => {
                        RangeBound::Inclusive(self.expression()?)
                    }
                );
                self.consume(Kind::To)?;
                let end = {
                    let expr = self.expression()?;
                    println!("EXpr is: {:?}", expr);
                    match_toks!(self,
                        k =>
                        return Err(ParseError::ExpectedTokens(
                            vec![Kind::RightParen, Kind::RightBracket],
                            k,
                            ident.line(),
                        ).into()),
                        Kind::RightParen => {
                            RangeBound::Exclusive(expr)
                        },
                        Kind::RightBracket => {
                            RangeBound::Inclusive(expr)
                        }
                    )
                };

                let body = self.statement()?;

                Ok(Stmt::For(Box::new(ForLoop::new((ident, usize::MAX).into(), (start, end), vec![body]))))
            },
            Kind::Until => {
                self.consume(Kind::LeftParen)?;
                let cond = self.expression()?;
                self.consume(Kind::RightParen)?;

                let body = self.statement()?;

                Ok(Stmt::While(Box::new(WhileLoop::new(cond, vec![body]))))
            }
        )
    }

    fn statement(&mut self) -> Result<Stmt> {
        match_toks!(self,
            _ => self.expression_statement(),
            Kind::LeftBoomerang => {
                self.inside_block += 1;
                let ret = self.block_statement();
                self.inside_block -= 1;
                ret
            },
            Kind::YaReckon => self.condition_statement(false),
            Kind::Whatabout => self.condition_statement(true),
            Kind::Gimme => self.print_statement(),
            Kind::Bail => self.return_statement(),
            Kind::MateFuckThis => {
                let tok = self.previous();
                self.consume(Kind::Semicolon)?;
                Ok(Stmt::Break(tok))
            },
            Kind::FuckinPiker => self.exit_statement(true),
            Kind::Cheers => self.exit_statement(false),
            Kind::Import => self.import_statement()
        )
    }

    fn import_statement(&mut self) -> Result<Stmt> {
        let ident = self.consume_ident()?;
        self.consume(Kind::Semicolon)?;

        Ok(Stmt::Import(ident))
    }

    fn exit_statement(&mut self, fuckinpiker: bool) -> Result<Stmt> {
        if fuckinpiker {
            self.consume(Kind::Semicolon)?;
        }
        Ok(Stmt::Exit(fuckinpiker))
    }

    /// Parse `block_statement`, expects the opening boomerang
    /// has already been consumed
    fn block_statement(&mut self) -> Result<Stmt> {
        let mut vec: Vec<Stmt> = Vec::new();

        while !self.match_tok(Kind::RightBoomerang) {
            vec.push(self.declaration()?);
        }

        Ok(Stmt::Block(vec))
    }

    fn condition_statement(&mut self, is_else: bool) -> Result<Stmt> {
        if self.inside_if == 0 && is_else {
            return Err(ParseError::InvalidWhatabout(self.previous().line()).into());
        }

        self.inside_if += 1;

        // No condition, so inside the last else
        if is_else && self.match_tok(Kind::QuestionMark) {
            self.inside_if -= 1;
            return Ok(Stmt::If(If {
                // Make it an always true if condition so it always executes
                cond: ExprNode::new(Expr::Literal(Value::Bool(true)), self.previous().line()),
                then: Box::new(self.statement()?),
                else_: None,
            }));
        }

        let cond = self.expression()?;
        let peek = self.peek();
        let kind = peek.kind();
        let line = peek.line();

        match kind {
            Kind::QuestionMark => {
                self.consume(Kind::QuestionMark)?;
                let then = Box::new(self.statement()?);
                let else_: Option<Box<Stmt>> = if self.peek().kind() == Kind::Whatabout {
                    Some(Box::new(self.statement()?))
                } else {
                    None
                };

                self.inside_if -= 1;
                Ok(Stmt::If(If::new(cond, then, else_)))
            }
            Kind::Isa => {
                self.consume(Kind::Isa)?;
                self.consume(Kind::LeftBoomerang)?;
                self.inside_block += 1;
                let ret = self.match_branches();
                self.inside_block -= 1;
                let (branches, default) = ret?;

                Ok(Stmt::Match(Match::new(cond, branches, default)))
            }
            other => {
                Err(
                    ParseError::ExpectedTokens(vec![Kind::QuestionMark, Kind::Isa], other, line)
                        .into(),
                )
            }
        }
    }

    fn print_statement(&mut self) -> Result<Stmt> {
        let stmt = Stmt::Print(self.expression()?);

        self.consume(Kind::Semicolon)?;

        Ok(stmt)
    }

    fn match_branches(&mut self) -> Result<(Vec<MatchBranch>, Option<MatchBranch>)> {
        let mut vec: Vec<MatchBranch> = Vec::new();
        let mut default: Option<MatchBranch> = None;

        while !self.match_tok(Kind::RightBoomerang) {
            let peek = self.peek();
            let val: Option<Pattern> = peek.clone().into();
            let val = if let Some(val) = val {
                let _ = self.advance();
                val
            } else {
                return Err(ParseError::ExpectedTokens(
                    vec![
                        Kind::Number(420.into()),
                        Kind::String("any string literal".into()),
                        Kind::NahYeah,
                        Kind::YeahNah,
                        Kind::BuggerAll,
                        Kind::Ident("any identifier".into()),
                    ],
                    peek.kind(),
                    peek.line(),
                )
                .into());
            };

            self.consume(Kind::Tilde)?;

            let body: Vec<Stmt> = self.statement()?.into();

            let branch = MatchBranch::new(val.clone(), body, peek.line());
            if let Pattern::Var(_) = val {
                if default.is_some() {
                    return Err(ParseError::TooManyMatchDefaultBranches(branch.line()).into());
                }
                default = Some(branch)
            } else {
                vec.push(branch)
            }
        }

        Ok((vec, default))
    }

    fn return_statement(&mut self) -> Result<Stmt> {
        let tok = self.previous();
        if self.match_tok(Kind::Semicolon) {
            Ok(Stmt::Return(tok, None))
        } else {
            let stmt = Stmt::Return(tok, Some(self.expression()?));
            self.consume(Kind::Semicolon)?;
            Ok(stmt)
        }
    }

    fn expression_statement(&mut self) -> Result<Stmt> {
        let expr = self.expression()?;

        self.consume(Kind::Semicolon)?;

        Ok(Stmt::Expr(expr))
    }

    fn expression(&mut self) -> Result<ExprNode> {
        self.assignment()
    }

    fn assignment(&mut self) -> Result<ExprNode> {
        let expr = self.or()?;
        match self.peek().kind {
            Kind::Assign => {
                let equals = self.advance();
                let initializer = self.expression()?;

                if let Expr::Var(v) = expr.expr() {
                    let assign = Expr::Assign(v.clone(), Box::new(initializer));
                    return Ok(ExprNode::new(assign, expr.line()));
                }

                Err(ParseError::InvalidAssigment(equals).into())
            }
            _ => Ok(expr),
        }
    }

    fn or(&mut self) -> Result<ExprNode> {
        let mut expr = self.and()?;

        while self.match_tok(Kind::Or) {
            let tok = self.previous();
            let right = self.and()?;
            expr = ExprNode::new(
                Expr::Logical(Box::new(expr), LogicalOp::Or, Box::new(right)),
                tok.line(),
            );
        }

        Ok(expr)
    }

    fn and(&mut self) -> Result<ExprNode> {
        let mut expr = self.equality()?;

        while self.match_tok(Kind::And) {
            let tok = self.previous();
            let right = self.equality()?;

            expr = ExprNode::new(
                Expr::Logical(Box::new(expr), LogicalOp::And, Box::new(right)),
                tok.line(),
            );
        }

        Ok(expr)
    }

    fn equality(&mut self) -> Result<ExprNode> {
        let mut left = self.comparison()?;
        let line = left.line();

        while matches!(self.peek().kind(), Kind::BangEqual | Kind::Equals) {
            let op: Option<BinaryOp> = self.advance().kind().into();
            let right = self.comparison()?;

            left = ExprNode::new(
                Expr::Binary(Box::new(left), op.unwrap(), Box::new(right)),
                line,
            );
        }

        Ok(left)
    }

    fn comparison(&mut self) -> Result<ExprNode> {
        let mut left = self.term()?;
        let line = left.line();

        while matches!(
            self.peek().kind(),
            Kind::RightBoomerang | Kind::GTE | Kind::LeftBoomerang | Kind::LTE
        ) {
            let kind = self.advance().kind();
            let op: Option<BinaryOp> = kind.clone().into();

            let right = match self.term() {
                Ok(term) => term,
                Err(e) => {
                    if self.inside_block > 0 {
                        return Err(ParseError::Any(
                            line,
                            "OI MATE, DID YA FORGET A SEMI-COLON OR AN OPERAND??".into(),
                        )
                        .into());
                    }
                    return Err(e);
                }
            };

            left = ExprNode::new(
                Expr::Binary(Box::new(left), op.unwrap(), Box::new(right)),
                line,
            );
        }

        Ok(left)
    }

    fn term(&mut self) -> Result<ExprNode> {
        let mut left = self.factor()?;
        let line = left.line();

        while matches!(self.peek().kind(), Kind::Minus | Kind::Plus) {
            let op: Option<BinaryOp> = self.advance().kind().into();
            let right = self.factor()?;

            left = ExprNode::new(
                Expr::Binary(Box::new(left), op.unwrap(), Box::new(right)),
                line,
            );
        }

        Ok(left)
    }

    fn factor(&mut self) -> Result<ExprNode> {
        let mut left = self.unary()?;
        let line = left.line();

        while matches!(
            self.peek().kind(),
            Kind::Slash | Kind::Asterisk | Kind::Modulo
        ) {
            let op: Option<BinaryOp> = self.advance().kind().into();
            let right = self.unary()?;

            left = ExprNode::new(
                Expr::Binary(Box::new(left), op.unwrap(), Box::new(right)),
                line,
            );
        }

        Ok(left)
    }

    fn unary(&mut self) -> Result<ExprNode> {
        match self.peek().kind() {
            Kind::Minus | Kind::Bang => {
                let tok = self.advance();
                let op: Option<UnaryOp> = tok.kind().into();
                let right = self.unary()?;
                Ok(ExprNode::new(
                    Expr::Unary(op.unwrap(), Box::new(right)),
                    tok.line(),
                ))
            }
            _ => self.call(),
        }
    }

    fn call(&mut self) -> Result<ExprNode> {
        let mut expr = self.primary()?;

        while self.match_tok(Kind::LeftParen) {
            expr = self.finish_call(expr)?
        }

        Ok(expr)
    }

    fn finish_call(&mut self, callee: ExprNode) -> Result<ExprNode> {
        let mut args: Vec<ExprNode> = Vec::new();

        if !self.check(Kind::RightParen) {
            loop {
                if args.len() > MAX_ARITY {
                    return Err(ParseError::TooManyArguments(callee.line()).into());
                }
                args.push(self.expression()?);

                if !self.match_tok(Kind::Comma) {
                    break;
                }
            }
        }

        let paren = self.consume(Kind::RightParen)?;
        let line = paren.line();

        Ok(ExprNode::new(
            Expr::Call(Box::new(callee), paren, args),
            line,
        ))
    }

    fn primary(&mut self) -> Result<ExprNode> {
        let next = self.advance();
        let line = next.line();

        let expr = match next.kind() {
            Kind::Number(num) => Expr::Literal(num.into()),
            Kind::String(s) => Expr::Literal(s.into()),
            Kind::NahYeah => Expr::Literal(true.into()),
            Kind::YeahNah => Expr::Literal(false.into()),
            Kind::BuggerAll => Expr::Literal(Value::Nil),
            Kind::Ident(name) => Expr::Var(Var::new(Ident::new(name, line), usize::MAX)),
            Kind::LeftParen => {
                let expr = self.expression()?;
                self.consume(Kind::RightParen)?;
                Expr::Grouping(Box::new(expr))
            }
            k => {
                // self.current -= 1;
                // panic!("k: {:?}", k);
                return Err(ParseError::ExpectPrimary(line, k).into());
            }
        };

        Ok(ExprNode::new(expr, line))
    }

    fn consume(&mut self, kind: Kind) -> Result<Token> {
        if self.check(kind.clone()) {
            return Ok(self.advance());
        }

        Err((ParseError::UnexpectedToken(kind, self.peek().kind(), self.peek().line())).into())
    }

    fn consume_ident(&mut self) -> Result<Ident> {
        let tok = self.peek();
        match tok.kind() {
            Kind::Ident(name) => {
                let _ = self.advance();
                Ok(Ident::new(name, tok.line()))
            }
            k => Err(ParseError::UnexpectedToken(Kind::Ident("any".into()), k, tok.line()).into()),
        }
    }

    fn consume_program_start(&mut self) -> Result<()> {
        while !self.is_at_end() {
            if self.match_tok(Kind::GdayMate) {
                return Ok(());
            }
            self.advance();
        }

        Err(ParseError::ExpectProgramStart.into())
    }

    fn match_tok(&mut self, kind: Kind) -> bool {
        if self.peek().kind() == kind {
            let _ = self.advance();
            true
        } else {
            false
        }
    }

    fn check(&self, kind: Kind) -> bool {
        if self.is_at_end() {
            return false;
        }
        self.peek().kind() == kind
    }

    fn peek(&self) -> Token {
        self.tokens[self.current].clone()
    }

    /// Peek `n` tokens ahead
    fn peek_n(&self, n: usize) -> Token {
        if self.current + n >= self.tokens.len() {
            self.tokens[self.tokens.len() - 1].clone()
        } else {
            self.tokens[self.current + n].clone()
        }
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn previous(&self) -> Token {
        self.tokens[self.current - 1].clone()
    }

    fn back(&mut self, pos: usize) {
        self.current = pos;
    }

    fn is_at_end(&self) -> bool {
        self.peek().kind() == Kind::EOF
    }
}
