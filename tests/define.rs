use armalint;
use armalint::config::{Node, Statement};

const FILE: &str = "tests/define.cpp";
const FILENAME: &str = "define.cpp";

#[test]
fn parse() {
    let content = std::fs::read_to_string(FILE).unwrap();
    let ast = armalint::config::parse(FILENAME, &content).unwrap();
    use armalint::config::{Node, Statement};
    assert_eq!(
        ast.config.statement,
        Statement::Config(vec![
            Node {
                file: FILENAME.to_string(),
                start: (1, 1),
                end: (1, 25),
                statement: Statement::Define {
                    ident: "username".to_string(),
                    value: Box::new(Node {
                        file: FILENAME.to_string(),
                        start: (1, 19),
                        end: (1, 24),
                        statement: Statement::Str("Brett".to_string())
                    })
                }
            },
            Node {
                file: FILENAME.to_string(),
                start: (3, 1),
                end: (5, 2),
                statement: Statement::Class {
                    ident: Box::new(Node {
                        file: FILENAME.to_string(),
                        start: (3, 7),
                        end: (3, 11),
                        statement: Statement::Ident("User".to_string())
                    }),
                    extends: None,
                    props: vec![Node {
                        file: FILENAME.to_string(),
                        start: (4, 5),
                        end: (4, 20),
                        statement: Statement::Property {
                            ident: Box::new(Node {
                                file: FILENAME.to_string(),
                                start: (4, 5),
                                end: (4, 9),
                                statement: Statement::Ident("name".to_string())
                            }),
                            value: Box::new(Node {
                                file: FILENAME.to_string(),
                                start: (4, 12),
                                end: (4, 20),
                                statement: Statement::Unquoted(vec![
                                    Node {
                                        file: FILENAME.to_string(),
                                        start: (4, 12),
                                        end: (4, 13),
                                        statement: Statement::Char('u')
                                    },
                                    Node {
                                        file: FILENAME.to_string(),
                                        start: (4, 13),
                                        end: (4, 14),
                                        statement: Statement::Char('s')
                                    },
                                    Node {
                                        file: FILENAME.to_string(),
                                        start: (4, 14),
                                        end: (4, 15),
                                        statement: Statement::Char('e')
                                    },
                                    Node {
                                        file: FILENAME.to_string(),
                                        start: (4, 15),
                                        end: (4, 16),
                                        statement: Statement::Char('r')
                                    },
                                    Node {
                                        file: FILENAME.to_string(),
                                        start: (4, 16),
                                        end: (4, 17),
                                        statement: Statement::Char('n')
                                    },
                                    Node {
                                        file: FILENAME.to_string(),
                                        start: (4, 17),
                                        end: (4, 18),
                                        statement: Statement::Char('a')
                                    },
                                    Node {
                                        file: FILENAME.to_string(),
                                        start: (4, 18),
                                        end: (4, 19),
                                        statement: Statement::Char('m')
                                    },
                                    Node {
                                        file: FILENAME.to_string(),
                                        start: (4, 19),
                                        end: (4, 20),
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
    let processed_ast = preprocessor.process(ast).unwrap();
    assert_eq!(
        processed_ast.config.statement,
        Statement::Config(vec![
            Node {
                file: FILENAME.to_string(),
                start: (1, 1),
                end: (1, 25),
                statement: Statement::Define {
                    ident: "username".to_string(),
                    value: Box::new(Node {
                        file: FILENAME.to_string(),
                        start: (1, 19),
                        end: (1, 24),
                        statement: Statement::Str("Brett".to_string())
                    })
                }
            },
            Node {
                file: FILENAME.to_string(),
                start: (3, 1),
                end: (5, 2),
                statement: Statement::Class {
                    ident: Box::new(Node {
                        file: FILENAME.to_string(),
                        start: (3, 7),
                        end: (3, 11),
                        statement: Statement::Ident("User".to_string())
                    }),
                    extends: None,
                    props: vec![Node {
                        file: FILENAME.to_string(),
                        start: (4, 5),
                        end: (4, 20),
                        statement: Statement::Property {
                            ident: Box::new(Node {
                                file: FILENAME.to_string(),
                                start: (4, 5),
                                end: (4, 9),
                                statement: Statement::Ident("name".to_string())
                            }),
                            value: Box::new(Node {
                                file: FILENAME.to_string(),
                                start: (4, 12),
                                end: (4, 20),
                                statement: Statement::Processed(
                                    Box::new(Statement::Defined(
                                        Box::new(Node {
                                            file: FILENAME.to_string(),
                                            start: (1, 19),
                                            end: (1, 24),
                                            statement: Statement::Str("Brett".to_string()),
                                        }),
                                        Box::new(Node {
                                            file: FILENAME.to_string(),
                                            start: (4, 12),
                                            end: (4, 20),
                                            statement: Statement::Unquoted(vec![
                                                Node {
                                                    file: FILENAME.to_string(),
                                                    start: (4, 12),
                                                    end: (4, 13),
                                                    statement: Statement::Char('u'),
                                                },
                                                Node {
                                                    file: FILENAME.to_string(),
                                                    start: (4, 13),
                                                    end: (4, 14),
                                                    statement: Statement::Char('s'),
                                                },
                                                Node {
                                                    file: FILENAME.to_string(),
                                                    start: (4, 14),
                                                    end: (4, 15),
                                                    statement: Statement::Char('e'),
                                                },
                                                Node {
                                                    file: FILENAME.to_string(),
                                                    start: (4, 15),
                                                    end: (4, 16),
                                                    statement: Statement::Char('r'),
                                                },
                                                Node {
                                                    file: FILENAME.to_string(),
                                                    start: (4, 16),
                                                    end: (4, 17),
                                                    statement: Statement::Char('n'),
                                                },
                                                Node {
                                                    file: FILENAME.to_string(),
                                                    start: (4, 17),
                                                    end: (4, 18),
                                                    statement: Statement::Char('a'),
                                                },
                                                Node {
                                                    file: FILENAME.to_string(),
                                                    start: (4, 18),
                                                    end: (4, 19),
                                                    statement: Statement::Char('m'),
                                                },
                                                Node {
                                                    file: FILENAME.to_string(),
                                                    start: (4, 19),
                                                    end: (4, 20),
                                                    statement: Statement::Char('e'),
                                                }
                                            ])
                                        })
                                    )),
                                    Box::new(Statement::Unquoted(vec![
                                        Node {
                                            file: FILENAME.to_string(),
                                            start: (4, 12),
                                            end: (4, 13),
                                            statement: Statement::Char('u'),
                                        },
                                        Node {
                                            file: FILENAME.to_string(),
                                            start: (4, 13),
                                            end: (4, 14),
                                            statement: Statement::Char('s'),
                                        },
                                        Node {
                                            file: FILENAME.to_string(),
                                            start: (4, 14),
                                            end: (4, 15),
                                            statement: Statement::Char('e'),
                                        },
                                        Node {
                                            file: FILENAME.to_string(),
                                            start: (4, 15),
                                            end: (4, 16),
                                            statement: Statement::Char('r'),
                                        },
                                        Node {
                                            file: FILENAME.to_string(),
                                            start: (4, 16),
                                            end: (4, 17),
                                            statement: Statement::Char('n'),
                                        },
                                        Node {
                                            file: FILENAME.to_string(),
                                            start: (4, 17),
                                            end: (4, 18),
                                            statement: Statement::Char('a'),
                                        },
                                        Node {
                                            file: FILENAME.to_string(),
                                            start: (4, 18),
                                            end: (4, 19),
                                            statement: Statement::Char('m'),
                                        },
                                        Node {
                                            file: FILENAME.to_string(),
                                            start: (4, 19),
                                            end: (4, 20),
                                            statement: Statement::Char('e'),
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
    let processed = preprocessor.process(ast).unwrap();
    armalint::config::simplify::Config::from_ast(processed).unwrap();
}

#[test]
fn rapify() {
    let content = std::fs::read_to_string(FILE).unwrap();
    let ast = armalint::config::parse(FILENAME, &content).unwrap();
    let mut preprocessor = armalint::config::PreProcessor::new();
    let processed = preprocessor.process(ast).unwrap();
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
