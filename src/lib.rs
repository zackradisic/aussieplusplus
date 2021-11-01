use anyhow::Result;
use parser::parser::Parser;
use resolver::Resolver;
use runtime::Interpreter;

pub mod ast;
pub mod lexer;
pub mod parser;
pub mod resolver;
pub mod runtime;
pub mod token;
pub mod upside_down;

pub fn interpret(src: &str) -> Result<()> {
    let mut lex = lexer::Lexer::new(lexer::source::Regular::new(src.chars()));
    let (tokens, _) = lex.lex();

    let mut parser = parser::parser::Parser::new(tokens);
    let mut stmts = parser.parse()?;

    if Resolver::new().resolve(&mut stmts) {
        return Ok(());
    }

    let mut iptr = Interpreter::new();
    iptr.interpret(stmts)?;

    Ok(())
}

pub fn interpret_repl(src: &str, interpreter: &mut Interpreter, parser: &mut Parser) -> Result<()> {
    let mut lex = lexer::Lexer::new(lexer::source::Regular::new(src.chars()));
    let (tokens, _) = lex.lex();

    parser.reset(tokens);
    let mut stmts = parser.parse()?;

    if Resolver::new().resolve(&mut stmts) {
        return Ok(());
    }

    interpreter.interpret(stmts)
}

pub fn interpret_upside_down(src: &str) -> Result<()> {
    let mut lex = lexer::Lexer::new(lexer::source::UpsideDown::new(src.chars()));
    let (tokens, _) = lex.lex();

    let mut parser = parser::parser::Parser::new(tokens);
    let mut stmts = parser.parse()?;

    if Resolver::new().resolve(&mut stmts) {
        return Ok(());
    }

    let mut iptr = Interpreter::new();
    iptr.interpret(stmts)?;

    Ok(())
}
