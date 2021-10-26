use aussie_plus_plus::{
    lexer::{self, source::Source},
    token::{Kind, Token},
};

fn test_lexing_with_src<T: Source>(mut expected_tokens: Vec<Token>, expected_error: bool, iter: T) {
    expected_tokens.insert(0, Token::new(Kind::GdayMate, 1));
    let mut lexer = lexer::Lexer::new(iter);
    let (tokens, had_error) = lexer.lex();

    assert_eq!(had_error, expected_error);
    {
        for (i, token) in tokens.iter().enumerate() {
            if &expected_tokens[i] != token {
                println!("expected: {:?} but got {:?}", expected_tokens[i], token);
            }
            assert_eq!(expected_tokens[i].clone(), token.clone());
        }
    }
    assert_eq!(tokens, expected_tokens);
}

fn test_lexing_upside_down(src: &str, expected_tokens: Vec<Token>, expected_error: bool) {
    test_lexing_with_src(
        expected_tokens,
        expected_error,
        lexer::source::UpsideDown::new(src.chars()),
    )
}

fn test_lexing(src: &str, expected_tokens: Vec<Token>, expected_error: bool) {
    let src = "G'day mate! ".to_owned() + src;
    test_lexing_with_src(
        expected_tokens,
        expected_error,
        lexer::source::Regular::new(src.chars()),
    )
}

#[test]
pub fn test_lex_is() {
    test_lexing(
        "is is a",
        vec![
            Token::new(Kind::Is, 1),
            Token::new(Kind::Isa, 1),
            Token::new(Kind::EOF, 1),
        ],
        false,
    );
}

#[test]
pub fn test_upside_down() {
    test_lexing_upside_down(
        "¡***Ɔ SɹƎƎHƆ
        ;0Ɩ = ʎ NOʞƆƎɹ I
        ;ϛ = x NOʞƆƎɹ I
        ¡Ǝ┴∀W ⅄∀p,פ",
        vec![
            Token::new(Kind::IReckon, 2),
            Token::new(Kind::Ident("x".into()), 2),
            Token::new(Kind::Assign, 2),
            Token::new(Kind::Number(5f64), 2),
            Token::new(Kind::Semicolon, 2),
            Token::new(Kind::IReckon, 3),
            Token::new(Kind::Ident("y".into()), 3),
            Token::new(Kind::Assign, 3),
            Token::new(Kind::Number(10f64), 3),
            Token::new(Kind::Semicolon, 3),
            Token::new(Kind::Cheers, 4),
            Token::new(Kind::EOF, 5),
        ],
        false,
    );
}

#[test]
pub fn test_separation() {
    test_lexing(
        "BUGGERALL",
        vec![
            Token::new(Kind::Ident("BUGGERALL".into()), 1),
            Token::new(Kind::EOF, 1),
        ],
        false,
    );
}

#[test]
pub fn test_lex_keyword_casing() {
    let kinds = vec![
        Kind::Gimme,
        Kind::Isa,
        Kind::Cheers,
        Kind::Walkabout,
        Kind::GdayMate,
        Kind::IReckon,
        Kind::YaReckon,
        Kind::HardYakkaFor,
        Kind::Bail,
        Kind::YeahNah,
        Kind::NahYeah,
        Kind::BuggerAll,
    ];

    fn test_random_case(kind: &Kind, iterations: usize) {
        let mut random_case: String;
        for _ in 0..iterations {
            random_case = kind
                .literal()
                .chars()
                .into_iter()
                .map(|c| {
                    if rand::random() {
                        c.to_ascii_uppercase()
                    } else {
                        c.to_ascii_lowercase()
                    }
                })
                .collect();

            let expected = {
                if kind.clone() == Kind::Cheers {
                    vec![Token::new(Kind::Cheers, 1), Token::new(Kind::EOF, 2)]
                } else {
                    vec![Token::new(kind.clone(), 1), Token::new(Kind::EOF, 1)]
                }
            };

            test_lexing(random_case.as_str(), expected, false);
        }
    }

    for kind in kinds {
        test_random_case(&kind, 5);
    }
}

#[test]
pub fn test_lex_hard_yakka() {
    test_lexing(
        "the Hard yakka for dummyFunction ( x ) <
            bail \"dinkum\"
        >",
        vec![
            Token::new(Kind::HardYakkaFor, 1),
            Token::new(Kind::Ident("dummyFunction".into()), 1),
            Token::new(Kind::LeftParen, 1),
            Token::new(Kind::Ident("x".into()), 1),
            Token::new(Kind::RightParen, 1),
            Token::new(Kind::LeftBoomerang, 1),
            Token::new(Kind::Bail, 2),
            Token::new(Kind::String("dinkum".into()), 2),
            Token::new(Kind::RightBoomerang, 3),
            Token::new(Kind::EOF, 3),
        ],
        false,
    );
}

#[test]
pub fn test_lex_keywords() {
    test_lexing(
        "G'DAY MATE!
        I RECKON x = 5
        I RECKON y = 10
        CHEERS C***!
        I RECKON z = 12
        ",
        vec![
            Token::new(Kind::GdayMate, 1),
            Token::new(Kind::IReckon, 2),
            Token::new(Kind::Ident("x".into()), 2),
            Token::new(Kind::Assign, 2),
            Token::new(Kind::Number(5f64), 2),
            Token::new(Kind::IReckon, 3),
            Token::new(Kind::Ident("y".into()), 3),
            Token::new(Kind::Assign, 3),
            Token::new(Kind::Number(10f64), 3),
            Token::new(Kind::Cheers, 4),
            Token::new(Kind::EOF, 5),
        ],
        false,
    );
}

#[test]
fn test_lex_ya_reckon() {
    test_lexing(
        "Ya reckon x == 5 <
            bail Nah, yeah
        >
        bail Yeah, Nah",
        vec![
            Token::new(Kind::YaReckon, 1),
            Token::new(Kind::Ident("x".into()), 1),
            Token::new(Kind::Equals, 1),
            Token::new(Kind::Number(5f64), 1),
            Token::new(Kind::LeftBoomerang, 1),
            Token::new(Kind::Bail, 2),
            Token::new(Kind::NahYeah, 2),
            Token::new(Kind::RightBoomerang, 3),
            Token::new(Kind::Bail, 4),
            Token::new(Kind::YeahNah, 4),
            Token::new(Kind::EOF, 4),
        ],
        false,
    )
}

#[test]
fn test_lex_walkabout() {
    test_lexing(
        "WALKABOUT (I reckon x = 0; x < 5; x = x + 1) <
            x = x + 1
        >",
        vec![
            Token::new(Kind::Walkabout, 1),
            Token::new(Kind::LeftParen, 1),
            Token::new(Kind::IReckon, 1),
            Token::new(Kind::Ident("x".into()), 1),
            Token::new(Kind::Assign, 1),
            Token::new(Kind::Number(0f64), 1),
            Token::new(Kind::Semicolon, 1),
            Token::new(Kind::Ident("x".into()), 1),
            Token::new(Kind::LeftBoomerang, 1),
            Token::new(Kind::Number(5f64), 1),
            Token::new(Kind::Semicolon, 1),
            Token::new(Kind::Ident("x".into()), 1),
            Token::new(Kind::Assign, 1),
            Token::new(Kind::Ident("x".into()), 1),
            Token::new(Kind::Plus, 1),
            Token::new(Kind::Number(1f64), 1),
            Token::new(Kind::RightParen, 1),
            Token::new(Kind::LeftBoomerang, 1),
            Token::new(Kind::Ident("x".into()), 2),
            Token::new(Kind::Assign, 2),
            Token::new(Kind::Ident("x".into()), 2),
            Token::new(Kind::Plus, 2),
            Token::new(Kind::Number(1f64), 2),
            Token::new(Kind::RightBoomerang, 3),
            Token::new(Kind::EOF, 3),
        ],
        false,
    );
}

#[test]
pub fn test_operators() {
    test_lexing(
        "5 < 10",
        vec![
            Token::new(Kind::Number(5f64), 1),
            Token::new(Kind::LeftBoomerang, 1),
            Token::new(Kind::Number(10f64), 1),
            Token::new(Kind::EOF, 1),
        ],
        false,
    );

    test_lexing(
        "5 > 10",
        vec![
            Token::new(Kind::Number(5f64), 1),
            Token::new(Kind::RightBoomerang, 1),
            Token::new(Kind::Number(10f64), 1),
            Token::new(Kind::EOF, 1),
        ],
        false,
    );

    test_lexing(
        "5 <= 10",
        vec![
            Token::new(Kind::Number(5f64), 1),
            Token::new(Kind::LTE, 1),
            Token::new(Kind::Number(10f64), 1),
            Token::new(Kind::EOF, 1),
        ],
        false,
    );

    test_lexing(
        "5 >= 10",
        vec![
            Token::new(Kind::Number(5f64), 1),
            Token::new(Kind::GTE, 1),
            Token::new(Kind::Number(10f64), 1),
            Token::new(Kind::EOF, 1),
        ],
        false,
    );

    test_lexing(
        "5 + 10",
        vec![
            Token::new(Kind::Number(5f64), 1),
            Token::new(Kind::Plus, 1),
            Token::new(Kind::Number(10f64), 1),
            Token::new(Kind::EOF, 1),
        ],
        false,
    );

    test_lexing(
        "5 - 10",
        vec![
            Token::new(Kind::Number(5f64), 1),
            Token::new(Kind::Minus, 1),
            Token::new(Kind::Number(10f64), 1),
            Token::new(Kind::EOF, 1),
        ],
        false,
    );

    test_lexing(
        "5 * 10",
        vec![
            Token::new(Kind::Number(5f64), 1),
            Token::new(Kind::Asterisk, 1),
            Token::new(Kind::Number(10f64), 1),
            Token::new(Kind::EOF, 1),
        ],
        false,
    );

    test_lexing(
        "5 / 10",
        vec![
            Token::new(Kind::Number(5f64), 1),
            Token::new(Kind::Slash, 1),
            Token::new(Kind::Number(10f64), 1),
            Token::new(Kind::EOF, 1),
        ],
        false,
    );

    test_lexing(
        "5 == 10",
        vec![
            Token::new(Kind::Number(5f64), 1),
            Token::new(Kind::Equals, 1),
            Token::new(Kind::Number(10f64), 1),
            Token::new(Kind::EOF, 1),
        ],
        false,
    );

    test_lexing(
        "5 != 10",
        vec![
            Token::new(Kind::Number(5f64), 1),
            Token::new(Kind::BangEqual, 1),
            Token::new(Kind::Number(10f64), 1),
            Token::new(Kind::EOF, 1),
        ],
        false,
    );

    test_lexing(
        "5 && 10",
        vec![
            Token::new(Kind::Number(5f64), 1),
            Token::new(Kind::And, 1),
            Token::new(Kind::Number(10f64), 1),
            Token::new(Kind::EOF, 1),
        ],
        false,
    );

    test_lexing(
        "5 || 10",
        vec![
            Token::new(Kind::Number(5f64), 1),
            Token::new(Kind::Or, 1),
            Token::new(Kind::Number(10f64), 1),
            Token::new(Kind::EOF, 1),
        ],
        false,
    );

    test_lexing(
        "5 % 10",
        vec![
            Token::new(Kind::Number(5f64), 1),
            Token::new(Kind::Modulo, 1),
            Token::new(Kind::Number(10f64), 1),
            Token::new(Kind::EOF, 1),
        ],
        false,
    );

    test_lexing(
        "!Yeah, Nah",
        vec![
            Token::new(Kind::Bang, 1),
            Token::new(Kind::YeahNah, 1),
            Token::new(Kind::EOF, 1),
        ],
        false,
    );
}
