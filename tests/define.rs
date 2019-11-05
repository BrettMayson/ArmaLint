use armalint;
use armalint::config::{Node, Statement};

const FILE: &str = "tests/define.cpp";
const FILENAME: &str = "define.cpp";

#[test]
fn parse() {
    let content = std::fs::read_to_string(FILE).unwrap();
    let ast = armalint::config::parse(FILENAME, &content).unwrap();
    assert_eq!(
        ast.config.statement,
        Statement::Config(vec![
            Node {
                file: "define.cpp".to_string(),
                start: (0, (1, 1)),
                end: (24, (1, 25)),
                line: "#define username \"Brett\"".to_string(),
                statement: Statement::Define {
                    ident: "username".to_string(),
                    value: Box::new(Node {
                        file: "define.cpp".to_string(),
                        start: (18, (1, 19)),
                        end: (23, (1, 24)),
                        line: "Brett".to_string(),
                        statement: Statement::Str("Brett".to_string())
                    })
                }
            },
            Node {
                file: "define.cpp".to_string(),
                start: (26, (3, 1)),
                end: (61, (5, 2)),
                line: "class User {\n    name = username;\n}".to_string(),
                statement: Statement::Class {
                    ident: Box::new(Node {
                        file: "define.cpp".to_string(),
                        start: (32, (3, 7)),
                        end: (36, (3, 11)),
                        line: "User".to_string(),
                        statement: Statement::Ident("User".to_string())
                    }),
                    extends: None,
                    props: vec![Node {
                        file: "define.cpp".to_string(),
                        start: (43, (4, 5)),
                        end: (58, (4, 20)),
                        line: "name = username".to_string(),
                        statement: Statement::Property {
                            ident: Box::new(Node {
                                file: "define.cpp".to_string(),
                                start: (43, (4, 5)),
                                end: (47, (4, 9)),
                                line: "name".to_string(),
                                statement: Statement::Ident("name".to_string())
                            }),
                            value: Box::new(Node {
                                file: "define.cpp".to_string(),
                                start: (50, (4, 12)),
                                end: (58, (4, 20)),
                                line: "username".to_string(),
                                statement: Statement::Unquoted(vec![
                                    Node {
                                        file: "define.cpp".to_string(),
                                        start: (50, (4, 12)),
                                        end: (51, (4, 13)),
                                        line: "u".to_string(),
                                        statement: Statement::Char('u')
                                    },
                                    Node {
                                        file: "define.cpp".to_string(),
                                        start: (51, (4, 13)),
                                        end: (52, (4, 14)),
                                        line: "s".to_string(),
                                        statement: Statement::Char('s')
                                    },
                                    Node {
                                        file: "define.cpp".to_string(),
                                        start: (52, (4, 14)),
                                        end: (53, (4, 15)),
                                        line: "e".to_string(),
                                        statement: Statement::Char('e')
                                    },
                                    Node {
                                        file: "define.cpp".to_string(),
                                        start: (53, (4, 15)),
                                        end: (54, (4, 16)),
                                        line: "r".to_string(),
                                        statement: Statement::Char('r')
                                    },
                                    Node {
                                        file: "define.cpp".to_string(),
                                        start: (54, (4, 16)),
                                        end: (55, (4, 17)),
                                        line: "n".to_string(),
                                        statement: Statement::Char('n')
                                    },
                                    Node {
                                        file: "define.cpp".to_string(),
                                        start: (55, (4, 17)),
                                        end: (56, (4, 18)),
                                        line: "a".to_string(),
                                        statement: Statement::Char('a')
                                    },
                                    Node {
                                        file: "define.cpp".to_string(),
                                        start: (56, (4, 18)),
                                        end: (57, (4, 19)),
                                        line: "m".to_string(),
                                        statement: Statement::Char('m')
                                    },
                                    Node {
                                        file: "define.cpp".to_string(),
                                        start: (57, (4, 19)),
                                        end: (58, (4, 20)),
                                        line: "e".to_string(),
                                        statement: Statement::Char('e')
                                    }
                                ])
                            }),
                            expand: false
                        }
                    }]
                }
            }
        ])
    );
}

#[test]
fn preprocess() {
    let content = std::fs::read_to_string(FILE).unwrap();
    let ast = armalint::config::parse(FILENAME, &content).unwrap();
    let mut preprocessor = armalint::config::PreProcessor::new();
    let (processed, _) = preprocessor.process(ast).unwrap();
    assert_eq!(
        processed.config.statement,
        Statement::Config(vec![
            Node {
                file: "define.cpp".to_string(),
                start: (0, (1, 1)),
                end: (24, (1, 25)),
                line: "#define username \"Brett\"".to_string(),
                statement: Statement::Define {
                    ident: "username".to_string(),
                    value: Box::new(Node {
                        file: "define.cpp".to_string(),
                        start: (18, (1, 19)),
                        end: (23, (1, 24)),
                        line: "Brett".to_string(),
                        statement: Statement::Str("Brett".to_string())
                    })
                }
            },
            Node {
                file: "define.cpp".to_string(),
                start: (26, (3, 1)),
                end: (61, (5, 2)),
                line: "class User {\n    name = username;\n}".to_string(),
                statement: Statement::Class {
                    ident: Box::new(Node {
                        file: "define.cpp".to_string(),
                        start: (32, (3, 7)),
                        end: (36, (3, 11)),
                        line: "User".to_string(),
                        statement: Statement::Ident("User".to_string())
                    }),
                    extends: None,
                    props: vec![Node {
                        file: "define.cpp".to_string(),
                        start: (43, (4, 5)),
                        end: (58, (4, 20)),
                        line: "name = username".to_string(),
                        statement: Statement::Property {
                            ident: Box::new(Node {
                                file: "define.cpp".to_string(),
                                start: (43, (4, 5)),
                                end: (47, (4, 9)),
                                line: "name".to_string(),
                                statement: Statement::Ident("name".to_string())
                            }),
                            value: Box::new(Node {
                                file: "define.cpp".to_string(),
                                start: (50, (4, 12)),
                                end: (58, (4, 20)),
                                line: "username".to_string(),
                                statement: Statement::Processed(
                                    Box::new(Statement::Defined(
                                        Box::new(Node {
                                            file: "define.cpp".to_string(),
                                            start: (18, (1, 19)),
                                            end: (23, (1, 24)),
                                            line: "Brett".to_string(),
                                            statement: Statement::Str("Brett".to_string())
                                        }),
                                        Box::new(Node {
                                            file: "define.cpp".to_string(),
                                            start: (50, (4, 12)),
                                            end: (58, (4, 20)),
                                            line: "username".to_string(),
                                            statement: Statement::Unquoted(vec![
                                                Node {
                                                    file: "define.cpp".to_string(),
                                                    start: (50, (4, 12)),
                                                    end: (51, (4, 13)),
                                                    line: "u".to_string(),
                                                    statement: Statement::Char('u')
                                                },
                                                Node {
                                                    file: "define.cpp".to_string(),
                                                    start: (51, (4, 13)),
                                                    end: (52, (4, 14)),
                                                    line: "s".to_string(),
                                                    statement: Statement::Char('s')
                                                },
                                                Node {
                                                    file: "define.cpp".to_string(),
                                                    start: (52, (4, 14)),
                                                    end: (53, (4, 15)),
                                                    line: "e".to_string(),
                                                    statement: Statement::Char('e')
                                                },
                                                Node {
                                                    file: "define.cpp".to_string(),
                                                    start: (53, (4, 15)),
                                                    end: (54, (4, 16)),
                                                    line: "r".to_string(),
                                                    statement: Statement::Char('r')
                                                },
                                                Node {
                                                    file: "define.cpp".to_string(),
                                                    start: (54, (4, 16)),
                                                    end: (55, (4, 17)),
                                                    line: "n".to_string(),
                                                    statement: Statement::Char('n')
                                                },
                                                Node {
                                                    file: "define.cpp".to_string(),
                                                    start: (55, (4, 17)),
                                                    end: (56, (4, 18)),
                                                    line: "a".to_string(),
                                                    statement: Statement::Char('a')
                                                },
                                                Node {
                                                    file: "define.cpp".to_string(),
                                                    start: (56, (4, 18)),
                                                    end: (57, (4, 19)),
                                                    line: "m".to_string(),
                                                    statement: Statement::Char('m')
                                                },
                                                Node {
                                                    file: "define.cpp".to_string(),
                                                    start: (57, (4, 19)),
                                                    end: (58, (4, 20)),
                                                    line: "e".to_string(),
                                                    statement: Statement::Char('e')
                                                }
                                            ])
                                        })
                                    )),
                                    Box::new(Statement::Unquoted(vec![
                                        Node {
                                            file: "define.cpp".to_string(),
                                            start: (50, (4, 12)),
                                            end: (51, (4, 13)),
                                            line: "u".to_string(),
                                            statement: Statement::Char('u')
                                        },
                                        Node {
                                            file: "define.cpp".to_string(),
                                            start: (51, (4, 13)),
                                            end: (52, (4, 14)),
                                            line: "s".to_string(),
                                            statement: Statement::Char('s')
                                        },
                                        Node {
                                            file: "define.cpp".to_string(),
                                            start: (52, (4, 14)),
                                            end: (53, (4, 15)),
                                            line: "e".to_string(),
                                            statement: Statement::Char('e')
                                        },
                                        Node {
                                            file: "define.cpp".to_string(),
                                            start: (53, (4, 15)),
                                            end: (54, (4, 16)),
                                            line: "r".to_string(),
                                            statement: Statement::Char('r')
                                        },
                                        Node {
                                            file: "define.cpp".to_string(),
                                            start: (54, (4, 16)),
                                            end: (55, (4, 17)),
                                            line: "n".to_string(),
                                            statement: Statement::Char('n')
                                        },
                                        Node {
                                            file: "define.cpp".to_string(),
                                            start: (55, (4, 17)),
                                            end: (56, (4, 18)),
                                            line: "a".to_string(),
                                            statement: Statement::Char('a')
                                        },
                                        Node {
                                            file: "define.cpp".to_string(),
                                            start: (56, (4, 18)),
                                            end: (57, (4, 19)),
                                            line: "m".to_string(),
                                            statement: Statement::Char('m')
                                        },
                                        Node {
                                            file: "define.cpp".to_string(),
                                            start: (57, (4, 19)),
                                            end: (58, (4, 20)),
                                            line: "e".to_string(),
                                            statement: Statement::Char('e')
                                        }
                                    ]))
                                )
                            }),
                            expand: false
                        }
                    }]
                }
            }
        ])
    );
}

#[test]
fn simplify() {
    let content = std::fs::read_to_string(FILE).unwrap();
    let ast = armalint::config::parse(FILENAME, &content).unwrap();
    let mut preprocessor = armalint::config::PreProcessor::new();
    let (processed, _) = preprocessor.process(ast).unwrap();
    armalint::config::simplify::Config::from_ast(processed).unwrap();
}

#[test]
fn rapify() {
    let content = std::fs::read_to_string(FILE).unwrap();
    let ast = armalint::config::parse(FILENAME, &content).unwrap();
    let mut preprocessor = armalint::config::PreProcessor::new();
    let (processed, _) = preprocessor.process(ast).unwrap();
    let simple = armalint::config::simplify::Config::from_ast(processed).unwrap();
    let mut rapified = std::io::Cursor::new(Vec::new());
    simple.write_rapified(&mut rapified).unwrap();
    use std::io::Read;
    let mut test_against = Vec::new();
    std::fs::File::open("tests/define.bin")
        .unwrap()
        .read_to_end(&mut test_against)
        .unwrap();
    assert_eq!(rapified.get_ref(), &test_against);
}
