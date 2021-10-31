use aussie_plus_plus::{
    lexer::{source, Lexer},
    parser::parser::Parser,
    resolver::Resolver,
    runtime::Interpreter,
};

fn test_code(src: &str, expected: &str) {
    let expected = if expected.is_empty() {
        expected.to_owned()
    } else {
        expected.to_owned() + "\n"
    };
    let mut s = "G'DAY MATE! ".to_string();
    s.push_str(src);
    let mut lex = Lexer::new(source::Regular::new(s.chars()));
    let (tokens, _) = lex.lex();
    println!("Tokens: {:#?}", tokens);
    let mut parser = Parser::new(tokens);
    let mut stmts = parser.parse().unwrap();

    if Resolver::new().resolve(&mut stmts) {
        panic!("Resolver failed")
    }

    let mut buf: Vec<u8> = Vec::with_capacity(128);
    let mut iptr = Interpreter::new_with_writer(&mut buf);

    if let Err(e) = iptr.interpret(stmts) {
        panic!("Failed to interpret: {}", e);
    }

    println!("Testing expression: {}", s);
    match std::str::from_utf8(&buf) {
        Err(e) => panic!("Failed to read buffer: {}", e),
        Ok(s) => {
            assert_eq!(s, expected);
        }
    }
}

#[test]
fn test_imports() {
    test_code(
        "
        IMPOHT ME FUNC ChuckSomeDice;
        IMPOHT ME FUNC HitTheSack;
        IMPOHT ME FUNC GimmeTime;

        HitTheSack(100);
        ChuckSomeDice(0, 1);
        GimmeTime();
",
        "",
    );
}

#[test]
fn test_scopes() {
    test_code(
        "
        i reckon x = 5;
        <
            THE HARD YAKKA FOR testFunc IS () <
               GIMME x;
            >

            testFunc();
            i reckon x = 420;
            testFunc();
        >
",
        "5\n5",
    );
}

#[test]
fn test_while_loop() {
    test_code(
        "
        i reckon x = 0;
        i reckon i'll have a walkabout until (x > 3) <
            gimme x;
            x = x + 1;
        >",
        "0\n1\n2\n3",
    );

    test_code(
        "
        i reckon i'll have a walkabout until (Yeah, nah!) <
            gimme \"bloody oath!\";
            mate fuck this;
        >
        gimme \"fair dinkum\";
        ",
        "bloody oath!\nfair dinkum",
    );
}

// #[test]
// fn test_early_exit() {
//     test_code(
//         "i reckon x = 5;
//              ya reckon x == 5 ? FUCKINPIKER;
//              gimme \"this should not appear\";",
//         "",
//     );
// }

#[test]
fn test_functions() {
    test_code(
        "
    THE HARD YAKKA FOR fibonacci IS ( x ) <
        YA RECKON x <= 1 ? BAIL x; 

        BAIL fibonacci(x - 1) + fibonacci(x - 2);
    >
    GIMME fibonacci(10);
    ",
        "55",
    );
}

#[test]
fn test_break() {
    test_code(
        "
    I RECKON x IS A walkabout FROM [1 to 5] <
        YA RECKON x == 2 ? MATE FUCK THIS;
        GIMME \"iteration number: \" + x;
    >
    ",
        "iteration number: 1",
    );
}

#[test]
fn test_for_loop_ranges() {
    test_code(
        "
    I reckon x is a walkabout from [0 to 2] <
        gimme x;
    >
    ",
        "0\n1\n2",
    );

    test_code(
        "
    I reckon x is a walkabout from (0 to 2] <
        gimme x;
    >
    ",
        "1\n2",
    );

    test_code(
        "
    I reckon x is a walkabout from [0 to 2] <
        gimme x;
    >
    ",
        "0\n1\n2",
    );

    test_code(
        "
    I reckon x is a walkabout from [0 to 2) <
        gimme x;
    >
    ",
        "0\n1",
    );

    test_code(
        "
    I reckon x is a walkabout from (0 to 0) <
        gimme \"val: \" + x;
    >
    ",
        "",
    );

    test_code(
        "
    I reckon x is a walkabout from (0 to 0] <
        gimme \"val: \" + x;
    >
    ",
        "",
    );

    test_code(
        "
    I reckon x is a walkabout from (-1 to 1] <
        gimme x;
    >
    ",
        "0\n1",
    );

    // Variables as ranges
    test_code(
        "
        I reckon z = -1;
        I reckon y = 1;
        I reckon x is a walkabout from (z to y] <
            gimme x;
        >
    ",
        "0\n1",
    );
}

#[test]
fn test_vars() {
    test_code(
        "
    I RECKON x = 10;
    gimme x;
    x = 5;
    gimme x;
    ",
        "10\n5",
    );

    test_code(
        "I RECKON x = 10;
    <
        I RECKON x = 5;
        gimme x;
    >
    gimme x;
    ",
        "5\n10",
    );

    test_code(
        "I RECKON x = 10;
    <
        x = 5;
        gimme x;
    >
    gimme x;
    ",
        "5\n5",
    );
}

#[test]
fn test_match() {
    // Works with bools
    test_code(
        "i reckon x = 2;
        ya reckon x == 2 is a <
                    Nah, yeah! ~ gimme \"FARK\";
                    Yeah, nah! ~ gimme 420;
                >",
        "FARK",
    );

    // Works with numbers
    test_code(
        "i reckon x = 420;
        ya reckon x is a <
                    1 ~ gimme \"FARK\";
                    2 ~ gimme \"CARN\";
                    420 ~ gimme \"FAIR DINKUM\";
                >
                gimme x;",
        "FAIR DINKUM\n420",
    );

    // Works with strings
    test_code(
        "i reckon x = \"G'day mate\";
        ya reckon x is a <
            \"Strewth!\" ~ gimme \"Strewth!\";
            \"G'day mate\" ~ gimme \"G'day mate\";
        >
        ",
        "G'day mate",
    );

    // Works with nil
    test_code(
        "i reckon x = BUGGER ALL;
        ya reckon x is a <
            BuGGEr ALL ~ gimme bugger all;
            somethinElse ~ gimme somethinElse;
        >
        ",
        "bugger all",
    );

    // Default case
    test_code(
        "i reckon x = 42069;
        ya reckon x is a <
            1 ~ gimme \"fark!\";
            1 ~ gimme \"carn!\";
            somethinElse ~ gimme somethinElse;
        >
        ",
        "42069",
    );
}

#[test]
fn test_if() {
    test_code(
        "i reckon x = 5;
        ya reckon x == 5 ? <
            gimme \"fair dinkum mate!\";
        >",
        "fair dinkum mate!",
    );

    test_code(
        "i reckon x = 5;
        ya reckon x == 5 ? gimme \"fair dinkum mate!\";
        ",
        "fair dinkum mate!",
    );

    test_code(
        "i reckon x = 5;
    ya reckon x == 42 ? gimme \"strewth!!\"; 
    gimme \"lmao\";",
        "lmao",
    );

    test_code(
        "
YA RECKON 1 == 2 ? GIMME \"fark we broke maths!\";
WHATABOUT NAH, YEAH! == YEAH, NAH! ? GIMME \"strewth we broke boolean logic!\";
WHATABOUT ? GIMME \"the universe is okay\";",
        "the universe is okay",
    );

    test_code(
        "
YA RECKON 1 == 2 ? GIMME \"fark we broke maths!\";
WHATABOUT YEAH, NAH! == YEAH, NAH! ? GIMME \"lmao\";
WHATABOUT ? GIMME \"the universe is okay\";",
        "lmao",
    );
}

#[test]
fn test_ops() {
    test_code("gimme 5 + 2;", "7");
    test_code("gimme 5 - 2;", "3");
    test_code("gimme 5 * 2;", "10");
    test_code("gimme 5 / 2;", "2.5");
    test_code("gimme 5 > 2;", "Nah, yeah!");
    test_code("gimme 5 >= 2;", "Nah, yeah!");
    test_code("gimme 5 < 2;", "Yeah, nah!");
    test_code("gimme 5 <= 2;", "Yeah, nah!");
    test_code("gimme 5 == 2;", "Yeah, nah!");
    test_code("gimme 5 != 2;", "Nah, yeah!");
    test_code("gimme 4 % 2;", "0");
    test_code("gimme 5 % 2;", "1");

    test_code("gimme nah, yeah! && yeah, nah!;", "Yeah, nah!");
    test_code("gimme nah, yeah! && nah, yeah!;", "Nah, yeah!");
    test_code("gimme nah, yeah! || yeah, nah!;", "Nah, yeah!");
    test_code("gimme yeah, nah! || yeah, nah!;", "Yeah, nah!");

    test_code("gimme ((5 + 5) / 2) * 2;", "10");

    test_code("gimme 5 + 5 * 2 / 2;", "10");
}
