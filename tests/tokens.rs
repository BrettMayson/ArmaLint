use armalint;
use armalint::config::{Node, Statement};

const FILE: &str = "tests/tokens.cpp";
const FILENAME: &str = "tokens.cpp";

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
                end: (1, 19),
                statement: Statement::Define {
                    ident: "NAME".to_string(),
                    value: Box::new(Node {
                        file: FILENAME.to_string(),
                        start: (1, 14),
                        end: (1, 19),
                        statement: Statement::Unquoted(vec![
                            Node {
                                file: FILENAME.to_string(),
                                start: (1, 14),
                                end: (1, 15),
                                statement: Statement::Char('B')
                            },
                            Node {
                                file: FILENAME.to_string(),
                                start: (1, 15),
                                end: (1, 16),
                                statement: Statement::Char('r')
                            },
                            Node {
                                file: FILENAME.to_string(),
                                start: (1, 16),
                                end: (1, 17),
                                statement: Statement::Char('e')
                            },
                            Node {
                                file: FILENAME.to_string(),
                                start: (1, 17),
                                end: (1, 18),
                                statement: Statement::Char('t')
                            },
                            Node {
                                file: FILENAME.to_string(),
                                start: (1, 18),
                                end: (1, 19),
                                statement: Statement::Char('t')
                            }
                        ])
                    })
                }
            },
            Node {
                file: FILENAME.to_string(),
                start: (2, 1),
                end: (3, 1),
                statement: Statement::DefineMacro {
                    ident: "QUOTE".to_string(),
                    args: vec!["s".to_string()],
                    value: Box::new(Node {
                        file: FILENAME.to_string(),
                        start: (2, 18),
                        end: (3, 1),
                        statement: Statement::MacroBody("#s\n".to_string())
                    })
                }
            },
            Node {
                file: FILENAME.to_string(),
                start: (3, 1),
                end: (4, 1),
                statement: Statement::DefineMacro {
                    ident: "APPEND".to_string(),
                    args: vec!["a".to_string(), "b".to_string()],
                    value: Box::new(Node {
                        file: "tokens.cpp".to_string(),
                        start: (3, 21),
                        end: (4, 1),
                        statement: Statement::MacroBody("a##b\n".to_string())
                    })
                }
            },
            Node {
                file: FILENAME.to_string(),
                start: (5, 1),
                end: (5, 19),
                statement: Statement::Property {
                    ident: Box::new(Node {
                        file: FILENAME.to_string(),
                        start: (5, 1),
                        end: (5, 5),
                        statement: Statement::Ident("name".to_string())
                    }),
                    value: Box::new(Node {
                        file: FILENAME.to_string(),
                        start: (5, 8),
                        end: (5, 19),
                        statement: Statement::Unquoted(vec![Node {
                            file: FILENAME.to_string(),
                            start: (5, 8),
                            end: (5, 19),
                            statement: Statement::MacroCall {
                                ident: "QUOTE".to_string(),
                                args: vec![Node {
                                    file: FILENAME.to_string(),
                                    start: (5, 14),
                                    end: (5, 18),
                                    statement: Statement::MacroCallArg(vec![
                                        Node {
                                            file: FILENAME.to_string(),
                                            start: (5, 14),
                                            end: (5, 15),
                                            statement: Statement::Char('N')
                                        },
                                        Node {
                                            file: FILENAME.to_string(),
                                            start: (5, 15),
                                            end: (5, 16),
                                            statement: Statement::Char('A')
                                        },
                                        Node {
                                            file: FILENAME.to_string(),
                                            start: (5, 16),
                                            end: (5, 17),
                                            statement: Statement::Char('M')
                                        },
                                        Node {
                                            file: FILENAME.to_string(),
                                            start: (5, 17),
                                            end: (5, 18),
                                            statement: Statement::Char('E')
                                        }
                                    ])
                                }]
                            }
                        }])
                    }),
                    expand: false
                }
            },
            Node {
                file: FILENAME.to_string(),
                start: (6, 1),
                end: (6, 40),
                statement: Statement::Property {
                    ident: Box::new(Node {
                        file: FILENAME.to_string(),
                        start: (6, 1),
                        end: (6, 12),
                        statement: Statement::Ident("tagged_name".to_string())
                    }),
                    value: Box::new(Node {
                        file: FILENAME.to_string(),
                        start: (6, 15),
                        end: (6, 40),
                        statement: Statement::Unquoted(vec![Node {
                            file: FILENAME.to_string(),
                            start: (6, 15),
                            end: (6, 40),
                            statement: Statement::MacroCall {
                                ident: "QUOTE".to_string(),
                                args: vec![Node {
                                    file: FILENAME.to_string(),
                                    start: (6, 21),
                                    end: (6, 39),
                                    statement: Statement::MacroCallArg(vec![Node {
                                        file: FILENAME.to_string(),
                                        start: (6, 21),
                                        end: (6, 39),
                                        statement: Statement::MacroCall {
                                            ident: "APPEND".to_string(),
                                            args: vec![
                                                Node {
                                                    file: FILENAME.to_string(),
                                                    start: (6, 28),
                                                    end: (6, 33),
                                                    statement: Statement::MacroCallArg(vec![
                                                        Node {
                                                            file: FILENAME.to_string(),
                                                            start: (6, 28),
                                                            end: (6, 29),
                                                            statement: Statement::Char('U')
                                                        },
                                                        Node {
                                                            file: FILENAME.to_string(),
                                                            start: (6, 29),
                                                            end: (6, 30),
                                                            statement: Statement::Char('S')
                                                        },
                                                        Node {
                                                            file: FILENAME.to_string(),
                                                            start: (6, 30),
                                                            end: (6, 31),
                                                            statement: Statement::Char('E')
                                                        },
                                                        Node {
                                                            file: FILENAME.to_string(),
                                                            start: (6, 31),
                                                            end: (6, 32),
                                                            statement: Statement::Char('R')
                                                        },
                                                        Node {
                                                            file: FILENAME.to_string(),
                                                            start: (6, 32),
                                                            end: (6, 33),
                                                            statement: Statement::Char('_')
                                                        }
                                                    ])
                                                },
                                                Node {
                                                    file: FILENAME.to_string(),
                                                    start: (6, 34),
                                                    end: (6, 38),
                                                    statement: Statement::MacroCallArg(vec![
                                                        Node {
                                                            file: FILENAME.to_string(),
                                                            start: (6, 34),
                                                            end: (6, 35),
                                                            statement: Statement::Char('N')
                                                        },
                                                        Node {
                                                            file: FILENAME.to_string(),
                                                            start: (6, 35),
                                                            end: (6, 36),
                                                            statement: Statement::Char('A')
                                                        },
                                                        Node {
                                                            file: FILENAME.to_string(),
                                                            start: (6, 36),
                                                            end: (6, 37),
                                                            statement: Statement::Char('M')
                                                        },
                                                        Node {
                                                            file: FILENAME.to_string(),
                                                            start: (6, 37),
                                                            end: (6, 38),
                                                            statement: Statement::Char('E')
                                                        }
                                                    ])
                                                }
                                            ]
                                        }
                                    }])
                                }]
                            }
                        }])
                    }),
                    expand: false
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
                end: (1, 19),
                statement: Statement::Define {
                    ident: "NAME".to_string(),
                    value: Box::new(Node {
                        file: FILENAME.to_string(),
                        start: (1, 14),
                        end: (1, 19),
                        statement: Statement::Unquoted(vec![
                            Node {
                                file: FILENAME.to_string(),
                                start: (1, 14),
                                end: (1, 15),
                                statement: Statement::Char('B')
                            },
                            Node {
                                file: FILENAME.to_string(),
                                start: (1, 15),
                                end: (1, 16),
                                statement: Statement::Char('r')
                            },
                            Node {
                                file: FILENAME.to_string(),
                                start: (1, 16),
                                end: (1, 17),
                                statement: Statement::Char('e')
                            },
                            Node {
                                file: FILENAME.to_string(),
                                start: (1, 17),
                                end: (1, 18),
                                statement: Statement::Char('t')
                            },
                            Node {
                                file: FILENAME.to_string(),
                                start: (1, 18),
                                end: (1, 19),
                                statement: Statement::Char('t')
                            }
                        ])
                    })
                }
            },
            Node {
                file: FILENAME.to_string(),
                start: (2, 1),
                end: (3, 1),
                statement: Statement::DefineMacro {
                    ident: "QUOTE".to_string(),
                    args: vec!["s".to_string()],
                    value: Box::new(Node {
                        file: FILENAME.to_string(),
                        start: (2, 18),
                        end: (3, 1),
                        statement: Statement::MacroBody("#s\n".to_string())
                    })
                }
            },
            Node {
                file: FILENAME.to_string(),
                start: (3, 1),
                end: (4, 1),
                statement: Statement::DefineMacro {
                    ident: "APPEND".to_string(),
                    args: vec!["a".to_string(), "b".to_string()],
                    value: Box::new(Node {
                        file: FILENAME.to_string(),
                        start: (3, 21),
                        end: (4, 1),
                        statement: Statement::MacroBody("a##b\n".to_string())
                    })
                }
            },
            Node {
                file: FILENAME.to_string(),
                start: (5, 1),
                end: (5, 19),
                statement: Statement::Property {
                    ident: Box::new(Node {
                        file: FILENAME.to_string(),
                        start: (5, 1),
                        end: (5, 5),
                        statement: Statement::Ident("name".to_string())
                    }),
                    value: Box::new(Node {
                        file: FILENAME.to_string(),
                        start: (5, 8),
                        end: (5, 19),
                        statement: Statement::Processed(
                            Box::new(Statement::InternalStr("Brett".to_string())),
                            Box::new(Statement::Unquoted(vec![Node {
                                file: FILENAME.to_string(),
                                start: (5, 8),
                                end: (5, 19),
                                statement: Statement::MacroCall {
                                    ident: "QUOTE".to_string(),
                                    args: vec![Node {
                                        file: FILENAME.to_string(),
                                        start: (5, 14),
                                        end: (5, 18),
                                        statement: Statement::MacroCallArg(vec![
                                            Node {
                                                file: FILENAME.to_string(),
                                                start: (5, 14),
                                                end: (5, 15),
                                                statement: Statement::Char('N')
                                            },
                                            Node {
                                                file: FILENAME.to_string(),
                                                start: (5, 15),
                                                end: (5, 16),
                                                statement: Statement::Char('A')
                                            },
                                            Node {
                                                file: FILENAME.to_string(),
                                                start: (5, 16),
                                                end: (5, 17),
                                                statement: Statement::Char('M')
                                            },
                                            Node {
                                                file: FILENAME.to_string(),
                                                start: (5, 17),
                                                end: (5, 18),
                                                statement: Statement::Char('E')
                                            }
                                        ])
                                    }]
                                }
                            }]))
                        )
                    }),
                    expand: false
                }
            },
            Node {
                file: FILENAME.to_string(),
                start: (6, 1),
                end: (6, 40),
                statement: Statement::Property {
                    ident: Box::new(Node {
                        file: FILENAME.to_string(),
                        start: (6, 1),
                        end: (6, 12),
                        statement: Statement::Ident("tagged_name".to_string())
                    }),
                    value: Box::new(Node {
                        file: FILENAME.to_string(),
                        start: (6, 15),
                        end: (6, 40),
                        statement: Statement::Processed(
                            Box::new(Statement::InternalStr("USER_Brett".to_string())),
                            Box::new(Statement::Unquoted(vec![Node {
                                file: FILENAME.to_string(),
                                start: (6, 15),
                                end: (6, 40),
                                statement: Statement::MacroCall {
                                    ident: "QUOTE".to_string(),
                                    args: vec![Node {
                                        file: FILENAME.to_string(),
                                        start: (6, 21),
                                        end: (6, 39),
                                        statement: Statement::MacroCallArg(vec![Node {
                                            file: FILENAME.to_string(),
                                            start: (6, 21),
                                            end: (6, 39),
                                            statement: Statement::MacroCall {
                                                ident: "APPEND".to_string(),
                                                args: vec![
                                                    Node {
                                                        file: FILENAME.to_string(),
                                                        start: (6, 28),
                                                        end: (6, 33),
                                                        statement: Statement::MacroCallArg(vec![
                                                            Node {
                                                                file: FILENAME.to_string(),
                                                                start: (6, 28),
                                                                end: (6, 29),
                                                                statement: Statement::Char('U')
                                                            },
                                                            Node {
                                                                file: FILENAME.to_string(),
                                                                start: (6, 29),
                                                                end: (6, 30),
                                                                statement: Statement::Char('S')
                                                            },
                                                            Node {
                                                                file: FILENAME.to_string(),
                                                                start: (6, 30),
                                                                end: (6, 31),
                                                                statement: Statement::Char('E')
                                                            },
                                                            Node {
                                                                file: FILENAME.to_string(),
                                                                start: (6, 31),
                                                                end: (6, 32),
                                                                statement: Statement::Char('R')
                                                            },
                                                            Node {
                                                                file: FILENAME.to_string(),
                                                                start: (6, 32),
                                                                end: (6, 33),
                                                                statement: Statement::Char('_')
                                                            }
                                                        ])
                                                    },
                                                    Node {
                                                        file: FILENAME.to_string(),
                                                        start: (6, 34),
                                                        end: (6, 38),
                                                        statement: Statement::MacroCallArg(vec![
                                                            Node {
                                                                file: FILENAME.to_string(),
                                                                start: (6, 34),
                                                                end: (6, 35),
                                                                statement: Statement::Char('N')
                                                            },
                                                            Node {
                                                                file: FILENAME.to_string(),
                                                                start: (6, 35),
                                                                end: (6, 36),
                                                                statement: Statement::Char('A')
                                                            },
                                                            Node {
                                                                file: FILENAME.to_string(),
                                                                start: (6, 36),
                                                                end: (6, 37),
                                                                statement: Statement::Char('M')
                                                            },
                                                            Node {
                                                                file: FILENAME.to_string(),
                                                                start: (6, 37),
                                                                end: (6, 38),
                                                                statement: Statement::Char('E')
                                                            }
                                                        ])
                                                    }
                                                ]
                                            }
                                        }])
                                    }]
                                }
                            }]))
                        )
                    }),
                    expand: false
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
    let mut f = std::fs::File::create("tokens.test_output").unwrap();
    simple.write_rapified(&mut f).unwrap();
    use std::io::Read;
    let mut test_against = Vec::new();
    std::fs::File::open("tests/tokens.bin")
        .unwrap()
        .read_to_end(&mut test_against)
        .unwrap();
    assert_eq!(rapified.get_ref(), &test_against);
}
