use aussie_plus_plus::{
    ast::{
        BinaryOp, Expr, ExprNode, ForLoop, Ident, If, Match, MatchBranch, Pattern, RangeBound,
        Stmt, UnaryOp, Var, VarDecl,
    },
    lexer::{lexer, source},
    parser::parser,
    runtime::Value,
    token::{Kind, Token},
};

fn test_parse<T>(source: &str, check_fn: T)
where
    T: FnOnce(Vec<Stmt>),
{
    let source = "G'DAY MATE! ".to_owned() + source;
    let mut lex = lexer::Lexer::new(source::Regular::new(source.chars()));
    let (tokens, _) = lex.lex();
    let mut parser = parser::Parser::new(tokens);
    let stmts = parser.parse().unwrap();

    check_fn(stmts)
}

#[test]
fn test_parse_for_loop() {
    test_parse(
        "i reckon x is a walkabout from (0 to 100) <
            gimme x;
            >",
        |stmts| {
            let inner = Stmt::Print(ExprNode::new(Expr::Var(("x", 2, usize::MAX).into()), 2));
            let body = vec![Stmt::Block(vec![inner])];
            let range = (
                RangeBound::Exclusive(ExprNode::new(Expr::Literal(0.into()), 1)),
                RangeBound::Exclusive(ExprNode::new(Expr::Literal(100.into()), 1)),
            );
            assert_eq!(
                stmts[0],
                Stmt::For(Box::new(ForLoop::new(
                    Var::new(("x", 1).into(), usize::MAX),
                    range,
                    body
                )))
            )
        },
    );

    test_parse(
        "i reckon x is a walkabout from (0 to 100) <
            mate fuck this;
            >",
        |stmts| {
            let inner = Stmt::Break(Token::new(Kind::MateFuckThis, 2));
            let body = vec![Stmt::Block(vec![inner])];
            let range = (
                RangeBound::Exclusive(ExprNode::new(Expr::Literal(0.into()), 1)),
                RangeBound::Exclusive(ExprNode::new(Expr::Literal(100.into()), 1)),
            );
            assert_eq!(
                stmts[0],
                Stmt::For(Box::new(ForLoop::new(
                    Var::new(("x", 1).into(), usize::MAX),
                    range,
                    body
                )))
            )
        },
    );
}

#[test]
fn test_parse_assign() {
    test_parse(
        "i reckon x = 5;
    x = 10;
    ",
        |stmts| {
            assert_eq!(
                stmts[1],
                Stmt::Expr(ExprNode::new(
                    Expr::Assign(
                        Var::new(("x", 2).into(), usize::MAX),
                        Box::new(ExprNode::new(Expr::Literal(10.into()), 2))
                    ),
                    2
                ))
            )
        },
    );

    test_parse(
        "
        i reckon x = 5;
        i reckon y = x = 10;
        ",
        |stmts| {
            assert_eq!(
                stmts[1],
                Stmt::VarDecl(VarDecl {
                    ident: ("y", 3).into(),
                    initializer: Some(ExprNode::new(
                        Expr::Assign(
                            Var::new(("x", 3).into(), usize::MAX),
                            Box::new(ExprNode::new(Expr::Literal(10.into()), 3)),
                        ),
                        3,
                    )),
                    immutable: false
                })
            );
        },
    );
}

#[test]
fn test_parse_match() {
    test_parse(
        "ya reckon x == 2 is a <
                    Nah, yeah! ~ Bugger all;
                    Yeah, nah! ~ 1;
                >",
        |stmts| {
            let cond = ExprNode::new(
                Expr::Binary(
                    Box::new(ExprNode::new(Expr::Var(("x", 1, usize::MAX).into()), 1)),
                    BinaryOp::Equal,
                    Box::new(ExprNode::new(Expr::Literal(2.into()), 1)),
                ),
                1,
            );
            let branches = vec![
                MatchBranch::new(
                    {
                        let pat: Option<Pattern> = Kind::True.into();
                        pat.unwrap()
                    },
                    vec![Stmt::Expr(ExprNode::new(Expr::Literal(Value::Nil), 2))],
                    2,
                ),
                MatchBranch::new(
                    {
                        let pat: Option<Pattern> = Kind::False.into();
                        pat.unwrap()
                    },
                    vec![Stmt::Expr(ExprNode::new(Expr::Literal(1.into()), 3))],
                    3,
                ),
            ];
            assert_eq!(stmts[0], Stmt::Match(Match::new(cond, branches, None)));
        },
    );
}

#[test]
fn test_parse_block() {
    test_parse(
        "ya reckon x == 2 ? <
                  i reckon y;
                  i reckon z;
                >",
        |stmts| {
            let cond = ExprNode::new(
                Expr::Binary(
                    Box::new(ExprNode::new(Expr::Var(("x", 1, usize::MAX).into()), 1)),
                    BinaryOp::Equal,
                    Box::new(ExprNode::new(Expr::Literal(2.into()), 1)),
                ),
                1,
            );
            let then = Box::new(Stmt::Block(vec![
                Stmt::VarDecl(VarDecl {
                    ident: ("y", 2).into(),
                    initializer: None,
                    immutable: false,
                }),
                Stmt::VarDecl(VarDecl {
                    ident: ("z", 3).into(),
                    initializer: None,
                    immutable: false,
                }),
            ]));
            assert_eq!(
                stmts[0],
                Stmt::If(If {
                    cond,
                    then,
                    else_: None
                }),
            );
        },
    );

    test_parse(
        "ya reckon x == 2 is a <
                    Nah, yeah! ~ Bugger all;
                    Yeah, nah! ~ 1;
                >",
        |stmts| {
            let cond = ExprNode::new(
                Expr::Binary(
                    Box::new(ExprNode::new(Expr::Var(("x", 1, usize::MAX).into()), 1)),
                    BinaryOp::Equal,
                    Box::new(ExprNode::new(Expr::Literal(2.into()), 1)),
                ),
                1,
            );
            let branches = vec![
                MatchBranch::new(
                    {
                        let pat: Option<Pattern> = Kind::True.into();
                        pat.unwrap()
                    },
                    vec![Stmt::Expr(ExprNode::new(Expr::Literal(Value::Nil), 2))],
                    2,
                ),
                MatchBranch::new(
                    {
                        let pat: Option<Pattern> = Kind::False.into();
                        pat.unwrap()
                    },
                    vec![Stmt::Expr(ExprNode::new(Expr::Literal(1.into()), 3))],
                    3,
                ),
            ];
            assert_eq!(stmts[0], Stmt::Match(Match::new(cond, branches, None)));
        },
    );
}

#[test]
fn test_parse_var() {
    test_parse(
        "i reckon x = 2;
                i reckon y;",
        |stmts| {
            assert_eq!(
                stmts[0],
                Stmt::VarDecl(VarDecl {
                    ident: ("x", 1).into(),
                    initializer: Some(ExprNode::new(Expr::Literal(2.into()), 1)),
                    immutable: false,
                })
            );
            assert_eq!(
                stmts[1],
                Stmt::VarDecl(VarDecl {
                    ident: Ident::new("y".into(), 2),
                    initializer: None,
                    immutable: false
                })
            )
        },
    );

    test_parse("i reckon x = 5 + 2;", |stmts| {
        assert_eq!(
            stmts[0],
            Stmt::VarDecl(VarDecl {
                ident: Ident::new("x".into(), 1),
                initializer: Some(ExprNode::new(
                    Expr::Binary(
                        Box::new(ExprNode::new(Expr::Literal(5.into()), 1)),
                        BinaryOp::Plus,
                        Box::new(ExprNode::new(Expr::Literal(2.into()), 1)),
                    ),
                    1
                )),
                immutable: false,
            })
        );
    });
}

#[test]
fn test_parse_if() {
    test_parse("ya reckon 5 == 2 ? nah, yeah!;", |stmts| {
        assert_eq!(
            stmts[0],
            Stmt::If(If {
                cond: ExprNode::new(
                    Expr::Binary(
                        Box::new(ExprNode::new(Expr::Literal(5.into()), 1)),
                        BinaryOp::Equal,
                        Box::new(ExprNode::new(Expr::Literal(2.into()), 1)),
                    ),
                    1
                ),
                then: Box::new(Stmt::Expr(ExprNode::new(Expr::Literal(true.into()), 1))),
                else_: None,
            })
        );
    });
}

#[test]
fn test_parse_unary_op() {
    test_parse("GOOD ON YA 1;", |stmts| {
        assert_eq!(
            stmts[0],
            Stmt::Expr(ExprNode::new(
                Expr::Unary(
                    UnaryOp::Incr,
                    Box::new(ExprNode::new(Expr::Literal(1.into()), 1)),
                ),
                1
            ))
        );
    });

    test_parse("PULL YA HEAD IN 1;", |stmts| {
        assert_eq!(
            stmts[0],
            Stmt::Expr(ExprNode::new(
                Expr::Unary(
                    UnaryOp::Decr,
                    Box::new(ExprNode::new(Expr::Literal(1.into()), 1)),
                ),
                1
            ))
        );
    });

    test_parse("!1;", |stmts| {
        assert_eq!(
            stmts[0],
            Stmt::Expr(ExprNode::new(
                Expr::Unary(
                    UnaryOp::Bang,
                    Box::new(ExprNode::new(Expr::Literal(1.into()), 1)),
                ),
                1
            ))
        );
    });

    test_parse("!!1;", |stmts| {
        assert_eq!(
            stmts[0],
            Stmt::Expr(ExprNode::new(
                Expr::Unary(
                    UnaryOp::Bang,
                    Box::new(ExprNode::new(
                        Expr::Unary(
                            UnaryOp::Bang,
                            Box::new(ExprNode::new(Expr::Literal(1.into()), 1)),
                        ),
                        1,
                    ))
                ),
                1
            ))
        );
    });

    test_parse("!!!1;", |stmts| {
        assert_eq!(
            stmts[0],
            Stmt::Expr(ExprNode::new(
                Expr::Unary(
                    UnaryOp::Bang,
                    Box::new(ExprNode::new(
                        Expr::Unary(
                            UnaryOp::Bang,
                            Box::new(ExprNode::new(
                                Expr::Unary(
                                    UnaryOp::Bang,
                                    Box::new(ExprNode::new(Expr::Literal(1.into()), 1)),
                                ),
                                1,
                            )),
                        ),
                        1,
                    ))
                ),
                1
            ))
        );
    });
}

#[test]
fn test_parse_binary_op() {
    test_parse("1 + 2 + 3 + 4;", |stmts| {
        assert_eq!(
            stmts[0],
            Stmt::Expr(ExprNode::new(
                Expr::Binary(
                    Box::new(ExprNode::new(
                        Expr::Binary(
                            Box::new(ExprNode::new(
                                Expr::Binary(
                                    Box::new(ExprNode::new(Expr::Literal(1.into()), 1)),
                                    BinaryOp::Plus,
                                    Box::new(ExprNode::new(Expr::Literal(2.into()), 1)),
                                ),
                                1,
                            )),
                            BinaryOp::Plus,
                            Box::new(ExprNode::new(Expr::Literal(3.into()), 1)),
                        ),
                        1,
                    )),
                    BinaryOp::Plus,
                    Box::new(ExprNode::new(Expr::Literal(4.into()), 1)),
                ),
                1
            ))
        );
    });

    test_parse("1 + (2 + 3);", |stmts| {
        assert_eq!(
            stmts[0],
            Stmt::Expr(ExprNode::new(
                Expr::Binary(
                    Box::new(ExprNode::new(Expr::Literal(1.into()), 1)),
                    BinaryOp::Plus,
                    Box::new(ExprNode::new(
                        Expr::Grouping(Box::new(ExprNode::new(
                            Expr::Binary(
                                Box::new(ExprNode::new(Expr::Literal(2.into()), 1)),
                                BinaryOp::Plus,
                                Box::new(ExprNode::new(Expr::Literal(3.into()), 1)),
                            ),
                            1,
                        ))),
                        1
                    ))
                ),
                1
            ))
        );
    });

    test_parse("1 + 2 + 3;", |stmts| {
        assert_eq!(
            stmts[0],
            Stmt::Expr(ExprNode::new(
                Expr::Binary(
                    Box::new(ExprNode::new(
                        Expr::Binary(
                            Box::new(ExprNode::new(Expr::Literal(1.into()), 1)),
                            BinaryOp::Plus,
                            Box::new(ExprNode::new(Expr::Literal(2.into()), 1)),
                        ),
                        1,
                    )),
                    BinaryOp::Plus,
                    Box::new(ExprNode::new(Expr::Literal(3.into()), 1))
                ),
                1
            ))
        );
    });

    test_parse("1 + 2;", |stmts| {
        assert_eq!(
            stmts[0],
            Stmt::Expr(ExprNode::new(
                Expr::Binary(
                    Box::new(ExprNode::new(Expr::Literal(1.into()), 1)),
                    BinaryOp::Plus,
                    Box::new(ExprNode::new(Expr::Literal(2.into()), 1)),
                ),
                1
            ))
        );
    });

    test_parse("1 - 2;", |stmts| {
        assert_eq!(
            stmts[0],
            Stmt::Expr(ExprNode::new(
                Expr::Binary(
                    Box::new(ExprNode::new(Expr::Literal(1.into()), 1)),
                    BinaryOp::Minus,
                    Box::new(ExprNode::new(Expr::Literal(2.into()), 1)),
                ),
                1
            ))
        );
    });

    test_parse("1 / 2;", |stmts| {
        assert_eq!(
            stmts[0],
            Stmt::Expr(ExprNode::new(
                Expr::Binary(
                    Box::new(ExprNode::new(Expr::Literal(1.into()), 1)),
                    BinaryOp::Divide,
                    Box::new(ExprNode::new(Expr::Literal(2.into()), 1)),
                ),
                1
            ))
        );
    });

    test_parse("1 * 2;", |stmts| {
        assert_eq!(
            stmts[0],
            Stmt::Expr(ExprNode::new(
                Expr::Binary(
                    Box::new(ExprNode::new(Expr::Literal(1.into()), 1)),
                    BinaryOp::Multiply,
                    Box::new(ExprNode::new(Expr::Literal(2.into()), 1)),
                ),
                1
            ))
        );
    });

    test_parse("1 == 2;", |stmts| {
        assert_eq!(
            stmts[0],
            Stmt::Expr(ExprNode::new(
                Expr::Binary(
                    Box::new(ExprNode::new(Expr::Literal(1.into()), 1)),
                    BinaryOp::Equal,
                    Box::new(ExprNode::new(Expr::Literal(2.into()), 1)),
                ),
                1
            ))
        );
    });

    test_parse("1 != 2;", |stmts| {
        assert_eq!(
            stmts[0],
            Stmt::Expr(ExprNode::new(
                Expr::Binary(
                    Box::new(ExprNode::new(Expr::Literal(1.into()), 1)),
                    BinaryOp::NotEqual,
                    Box::new(ExprNode::new(Expr::Literal(2.into()), 1)),
                ),
                1
            ))
        );
    });

    test_parse("1 > 2;", |stmts| {
        assert_eq!(
            stmts[0],
            Stmt::Expr(ExprNode::new(
                Expr::Binary(
                    Box::new(ExprNode::new(Expr::Literal(1.into()), 1)),
                    BinaryOp::Greater,
                    Box::new(ExprNode::new(Expr::Literal(2.into()), 1)),
                ),
                1
            ))
        );
    });

    test_parse("1 >= 2;", |stmts| {
        assert_eq!(
            stmts[0],
            Stmt::Expr(ExprNode::new(
                Expr::Binary(
                    Box::new(ExprNode::new(Expr::Literal(1.into()), 1)),
                    BinaryOp::GreaterEqual,
                    Box::new(ExprNode::new(Expr::Literal(2.into()), 1)),
                ),
                1
            ))
        );
    });

    test_parse("1 < 2;", |stmts| {
        assert_eq!(
            stmts[0],
            Stmt::Expr(ExprNode::new(
                Expr::Binary(
                    Box::new(ExprNode::new(Expr::Literal(1.into()), 1)),
                    BinaryOp::Less,
                    Box::new(ExprNode::new(Expr::Literal(2.into()), 1)),
                ),
                1
            ))
        );
    });

    test_parse("1 <= 2;", |stmts| {
        assert_eq!(
            stmts[0],
            Stmt::Expr(ExprNode::new(
                Expr::Binary(
                    Box::new(ExprNode::new(Expr::Literal(1.into()), 1)),
                    BinaryOp::LessEqual,
                    Box::new(ExprNode::new(Expr::Literal(2.into()), 1)),
                ),
                1
            ))
        );
    });

    test_parse("1 % 2;", |stmts| {
        assert_eq!(
            stmts[0],
            Stmt::Expr(ExprNode::new(
                Expr::Binary(
                    Box::new(ExprNode::new(Expr::Literal(1.into()), 1)),
                    BinaryOp::Modulo,
                    Box::new(ExprNode::new(Expr::Literal(2.into()), 1)),
                ),
                1
            ))
        );
    });
}
