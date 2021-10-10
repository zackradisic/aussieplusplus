use aussie_plus_plus::{
    lexer::{source, Lexer},
    parser::parser::Parser,
    runtime::Interpreter,
};

fn main() {
    fn fibonacci(n: u32) {
        let code = "the hard yakka for fibonacci is ( x ) < ya reckon x <= 1 ? <
                bail x;
            >
            bail fibonacci(x - 1) + fibonacci(x - 2);
        >
        gimme fibonacci("
            .to_string()
            + &n.to_string()
            + &");".to_string();
        let mut lex = Lexer::new(source::Regular::new(code.chars()));
        let (tokens, _) = lex.lex();
        let mut parser = Parser::new(tokens);
        let stmts = parser.parse();

        let mut buf: Vec<u8> = Vec::with_capacity(128);
        let mut iptr = Interpreter::new_with_writer(&mut buf);

        if let Err(e) = iptr.interpret(stmts) {
            panic!("Failed to interpret: {}", e);
        }
    }

    fibonacci(30);
}
