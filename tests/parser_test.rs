use aussie_plus_plus::{
    ast::{BinaryOp, Expr, ExprNode, Ident, MatchBody, MatchBranch, Pattern, Stmt, UnaryOp},
    lexer::{lexer, source},
    parser::parser,
    runtime::Value,
    token::Kind,
};

fn test_parse<T>(source: &str, check_fn: T)
where
    T: FnOnce(Vec<Stmt>) -> (),
{
    let mut lex = lexer::Lexer::new(source::Regular::new(source.chars()));
    let (tokens, _) = lex.lex();
    let mut parser = parser::Parser::new(tokens);
    let stmts = parser.parse();

    check_fn(stmts)
}

#[test]
fn test_parse_match() {
    test_parse(
        "ya reckon x == 2 is a <
                    Nah, yeah ~ Bugger all
                    Yeah, nah ~ 1
                >",
        |stmts| {
            let cond = ExprNode::new(
                Expr::Binary(
                    Box::new(ExprNode::new(Expr::Var(("x", 1).into()), 1)),
                    BinaryOp::Equal,
                    Box::new(ExprNode::new(Expr::Literal(2.into()), 1)),
                ),
                1,
            );
            let branches = vec![
                MatchBranch::new(
                    {
                        let pat: Option<Pattern> = Kind::NahYeah.into();
                        pat.unwrap()
                    },
                    MatchBody::Expr(ExprNode::new(Expr::Literal(Value::Nil), 2)),
                    2,
                ),
                MatchBranch::new(
                    {
                        let pat: Option<Pattern> = Kind::YeahNah.into();
                        pat.unwrap()
                    },
                    MatchBody::Expr(ExprNode::new(Expr::Literal(1.into()), 3)),
                    3,
                ),
            ];
            assert_eq!(stmts[0], Stmt::Match(cond, branches));
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
                    Box::new(ExprNode::new(Expr::Var(("x", 1).into()), 1)),
                    BinaryOp::Equal,
                    Box::new(ExprNode::new(Expr::Literal(2.into()), 1)),
                ),
                1,
            );
            let block = Box::new(Stmt::Block(vec![
                Stmt::VarDecl(("y", 2).into(), None),
                Stmt::VarDecl(("z", 3).into(), None),
            ]));
            assert_eq!(stmts[0], Stmt::If(cond, block),);
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
                Stmt::VarDecl(
                    ("x", 2).into(),
                    Some(ExprNode::new(Expr::Literal(2.into()), 1))
                )
            );
            assert_eq!(stmts[1], Stmt::VarDecl(Ident::new("y".into(), 2), None))
        },
    );

    test_parse("i reckon x = 5 + 2;", |stmts| {
        assert_eq!(
            stmts[0],
            Stmt::VarDecl(
                Ident::new("x".into(), 1),
                Some(ExprNode::new(
                    Expr::Binary(
                        Box::new(ExprNode::new(Expr::Literal(5.into()), 1)),
                        BinaryOp::Plus,
                        Box::new(ExprNode::new(Expr::Literal(2.into()), 1)),
                    ),
                    1
                ))
            )
        );
    });
}

#[test]
fn test_parse_if() {
    test_parse("ya reckon 5 == 2 ? nah, yeah", |stmts| {
        assert_eq!(
            stmts[0],
            Stmt::If(
                ExprNode::new(
                    Expr::Binary(
                        Box::new(ExprNode::new(Expr::Literal(5.into()), 1)),
                        BinaryOp::Equal,
                        Box::new(ExprNode::new(Expr::Literal(2.into()), 1)),
                    ),
                    1
                ),
                Box::new(Stmt::Expr(ExprNode::new(Expr::Literal(true.into()), 1)))
            )
        );
    });
}

#[test]
fn test_parse_unary_op() {
    test_parse("!1", |stmts| {
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

    test_parse("!!1", |stmts| {
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

    test_parse("!!!1", |stmts| {
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
    test_parse("1 + 2 + 3 + 4", |stmts| {
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

    test_parse("1 + 2 + 3", |stmts| {
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

    test_parse("1 + 2", |stmts| {
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

    test_parse("1 - 2", |stmts| {
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

    test_parse("1 / 2", |stmts| {
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

    test_parse("1 * 2", |stmts| {
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

    test_parse("1 == 2", |stmts| {
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

    test_parse("1 != 2", |stmts| {
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

    test_parse("1 > 2", |stmts| {
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

    test_parse("1 >= 2", |stmts| {
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

    test_parse("1 < 2", |stmts| {
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

    test_parse("1 <= 2", |stmts| {
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
}
