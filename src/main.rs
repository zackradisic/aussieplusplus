use std::{
    fs,
    io::{self, BufRead},
    path::PathBuf,
};
use structopt::StructOpt;

use aussie_plus_plus::{
    lexer::{source, Lexer},
    parser::parser::Parser,
    runtime::Interpreter,
};

#[derive(StructOpt, Debug)]
#[structopt(name = "aussie++")]
struct Opt {
    /// Path to input file
    #[structopt(name = "File", parse(from_os_str))]
    filepath: Option<PathBuf>,
}

fn main() {
    let opt = Opt::from_args();

    let code: String;
    if let Some(filepath) = opt.filepath {
        code = fs::read_to_string(filepath).expect("failed to read file");
        let mut lex = Lexer::new(source::Regular::new(code.chars()));
        let (tokens, had_error) = lex.lex();
        if had_error {
            return;
        }
        let mut parser = Parser::new(tokens);
        let stmts = match parser.parse() {
            Ok(stmts) => stmts,
            Err(_) => {
                return;
            }
        };

        let mut buf: Vec<u8> = Vec::with_capacity(128);
        let mut iptr = Interpreter::new_with_writer(&mut buf);

        if let Err(e) = iptr.interpret(stmts) {
            panic!("Failed to interpret: {}", e);
        }

        return;
    }

    let stdin = io::stdin();
    let mut i = Interpreter::new();
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        let mut lex = Lexer::new(source::Regular::new(line.chars()));
        let (tokens, had_error) = lex.lex();
        if had_error {
            return;
        }
        let mut parser = Parser::new(tokens);
        let stmts = match parser.parse() {
            Ok(stmts) => stmts,
            Err(_) => {
                return;
            }
        };

        if let Err(e) = i.interpret(stmts) {
            panic!("Failed to interpret: {}", e);
        }
    }
}
