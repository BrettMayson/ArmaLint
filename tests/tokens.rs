use armalint;
use armalint::config::{Node, Statement};

const FILE: &str = "tests/tokens.cpp";
const FILENAME: &str = "tokens.cpp";

#[test]
fn parse() {
    let content = std::fs::read_to_string(FILE).unwrap();
    let ast = armalint::config::parse(FILENAME, &content).unwrap();
    assert_eq!(
        ast.config.statement,
        Statement::Config(vec![
            Node {
                file: "tokens.cpp".to_string(),
                start: (0, (1, 1)),
                end: (18, (1, 19)),
                line: "#define NAME Brett".to_string(),
                statement: Statement::Define {
                    ident: "NAME".to_string(),
                    value: Box::new(Node {
                        file: "tokens.cpp".to_string(),
                        start: (13, (1, 14)),
                        end: (18, (1, 19)),
                        line: "Brett".to_string(),
                        statement: Statement::Unquoted(vec![
                            Node {
                                file: "tokens.cpp".to_string(),
                                start: (13, (1, 14)),
                                end: (14, (1, 15)),
                                line: "B".to_string(),
                                statement: Statement::Char('B')
                            },
                            Node {
                                file: "tokens.cpp".to_string(),
                                start: (14, (1, 15)),
                                end: (15, (1, 16)),
                                line: "r".to_string(),
                                statement: Statement::Char('r')
                            },
                            Node {
                                file: "tokens.cpp".to_string(),
                                start: (15, (1, 16)),
                                end: (16, (1, 17)),
                                line: "e".to_string(),
                                statement: Statement::Char('e')
                            },
                            Node {
                                file: "tokens.cpp".to_string(),
                                start: (16, (1, 17)),
                                end: (17, (1, 18)),
                                line: "t".to_string(),
                                statement: Statement::Char('t')
                            },
                            Node {
                                file: "tokens.cpp".to_string(),
                                start: (17, (1, 18)),
                                end: (18, (1, 19)),
                                line: "t".to_string(),
                                statement: Statement::Char('t')
                            }
                        ])
                    })
                }
            },
            Node {
                file: "tokens.cpp".to_string(),
                start: (19, (2, 1)),
                end: (39, (3, 1)),
                line: "#define QUOTE(s) #s\n".to_string(),
                statement: Statement::DefineMacro {
                    ident: "QUOTE".to_string(),
                    args: vec!["s".to_string()],
                    value: Box::new(Node {
                        file: "tokens.cpp".to_string(),
                        start: (36, (2, 18)),
                        end: (39, (3, 1)),
                        line: "#s\n".to_string(),
                        statement: Statement::MacroBody("#s".to_string())
                    })
                }
            },
            Node {
                file: "tokens.cpp".to_string(),
                start: (39, (3, 1)),
                end: (64, (4, 1)),
                line: "#define APPEND(a,b) a##b\n".to_string(),
                statement: Statement::DefineMacro {
                    ident: "APPEND".to_string(),
                    args: vec!["a".to_string(), "b".to_string()],
                    value: Box::new(Node {
                        file: "tokens.cpp".to_string(),
                        start: (59, (3, 21)),
                        end: (64, (4, 1)),
                        line: "a##b\n".to_string(),
                        statement: Statement::MacroBody("a##b".to_string())
                    })
                }
            },
            Node {
                file: "tokens.cpp".to_string(),
                start: (65, (5, 1)),
                end: (83, (5, 19)),
                line: "name = QUOTE(NAME)".to_string(),
                statement: Statement::Property {
                    ident: Box::new(Node {
                        file: "tokens.cpp".to_string(),
                        start: (65, (5, 1)),
                        end: (69, (5, 5)),
                        line: "name".to_string(),
                        statement: Statement::Ident("name".to_string())
                    }),
                    value: Box::new(Node {
                        file: "tokens.cpp".to_string(),
                        start: (72, (5, 8)),
                        end: (83, (5, 19)),
                        line: "QUOTE(NAME)".to_string(),
                        statement: Statement::MacroCall {
                            ident: "QUOTE".to_string(),
                            args: vec![Node {
                                file: "tokens.cpp".to_string(),
                                start: (78, (5, 14)),
                                end: (82, (5, 18)),
                                line: "NAME".to_string(),
                                statement: Statement::MacroCallArg(vec![
                                    Node {
                                        file: "tokens.cpp".to_string(),
                                        start: (78, (5, 14)),
                                        end: (79, (5, 15)),
                                        line: "N".to_string(),
                                        statement: Statement::Char('N')
                                    },
                                    Node {
                                        file: "tokens.cpp".to_string(),
                                        start: (79, (5, 15)),
                                        end: (80, (5, 16)),
                                        line: "A".to_string(),
                                        statement: Statement::Char('A')
                                    },
                                    Node {
                                        file: "tokens.cpp".to_string(),
                                        start: (80, (5, 16)),
                                        end: (81, (5, 17)),
                                        line: "M".to_string(),
                                        statement: Statement::Char('M')
                                    },
                                    Node {
                                        file: "tokens.cpp".to_string(),
                                        start: (81, (5, 17)),
                                        end: (82, (5, 18)),
                                        line: "E".to_string(),
                                        statement: Statement::Char('E')
                                    }
                                ])
                            }]
                        }
                    }),
                    expand: false
                }
            },
            Node {
                file: "tokens.cpp".to_string(),
                start: (85, (6, 1)),
                end: (124, (6, 40)),
                line: "tagged_name = QUOTE(APPEND(USER_,NAME))".to_string(),
                statement: Statement::Property {
                    ident: Box::new(Node {
                        file: "tokens.cpp".to_string(),
                        start: (85, (6, 1)),
                        end: (96, (6, 12)),
                        line: "tagged_name".to_string(),
                        statement: Statement::Ident("tagged_name".to_string())
                    }),
                    value: Box::new(Node {
                        file: "tokens.cpp".to_string(),
                        start: (99, (6, 15)),
                        end: (124, (6, 40)),
                        line: "QUOTE(APPEND(USER_,NAME))".to_string(),
                        statement: Statement::MacroCall {
                            ident: "QUOTE".to_string(),
                            args: vec![Node {
                                file: "tokens.cpp".to_string(),
                                start: (105, (6, 21)),
                                end: (123, (6, 39)),
                                line: "APPEND(USER_,NAME)".to_string(),
                                statement: Statement::MacroCallArg(vec![Node {
                                    file: "tokens.cpp".to_string(),
                                    start: (105, (6, 21)),
                                    end: (123, (6, 39)),
                                    line: "APPEND(USER_,NAME)".to_string(),
                                    statement: Statement::MacroCall {
                                        ident: "APPEND".to_string(),
                                        args: vec![
                                            Node {
                                                file: "tokens.cpp".to_string(),
                                                start: (112, (6, 28)),
                                                end: (117, (6, 33)),
                                                line: "USER_".to_string(),
                                                statement: Statement::MacroCallArg(vec![
                                                    Node {
                                                        file: "tokens.cpp".to_string(),
                                                        start: (112, (6, 28)),
                                                        end: (113, (6, 29)),
                                                        line: "U".to_string(),
                                                        statement: Statement::Char('U')
                                                    },
                                                    Node {
                                                        file: "tokens.cpp".to_string(),
                                                        start: (113, (6, 29)),
                                                        end: (114, (6, 30)),
                                                        line: "S".to_string(),
                                                        statement: Statement::Char('S')
                                                    },
                                                    Node {
                                                        file: "tokens.cpp".to_string(),
                                                        start: (114, (6, 30)),
                                                        end: (115, (6, 31)),
                                                        line: "E".to_string(),
                                                        statement: Statement::Char('E')
                                                    },
                                                    Node {
                                                        file: "tokens.cpp".to_string(),
                                                        start: (115, (6, 31)),
                                                        end: (116, (6, 32)),
                                                        line: "R".to_string(),
                                                        statement: Statement::Char('R')
                                                    },
                                                    Node {
                                                        file: "tokens.cpp".to_string(),
                                                        start: (116, (6, 32)),
                                                        end: (117, (6, 33)),
                                                        line: "_".to_string(),
                                                        statement: Statement::Char('_')
                                                    }
                                                ])
                                            },
                                            Node {
                                                file: "tokens.cpp".to_string(),
                                                start: (118, (6, 34)),
                                                end: (122, (6, 38)),
                                                line: "NAME".to_string(),
                                                statement: Statement::MacroCallArg(vec![
                                                    Node {
                                                        file: "tokens.cpp".to_string(),
                                                        start: (118, (6, 34)),
                                                        end: (119, (6, 35)),
                                                        line: "N".to_string(),
                                                        statement: Statement::Char('N')
                                                    },
                                                    Node {
                                                        file: "tokens.cpp".to_string(),
                                                        start: (119, (6, 35)),
                                                        end: (120, (6, 36)),
                                                        line: "A".to_string(),
                                                        statement: Statement::Char('A')
                                                    },
                                                    Node {
                                                        file: "tokens.cpp".to_string(),
                                                        start: (120, (6, 36)),
                                                        end: (121, (6, 37)),
                                                        line: "M".to_string(),
                                                        statement: Statement::Char('M')
                                                    },
                                                    Node {
                                                        file: "tokens.cpp".to_string(),
                                                        start: (121, (6, 37)),
                                                        end: (122, (6, 38)),
                                                        line: "E".to_string(),
                                                        statement: Statement::Char('E')
                                                    }
                                                ])
                                            }
                                        ]
                                    }
                                }])
                            }]
                        }
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
    let processed = preprocessor.process(ast).unwrap();
    assert_eq!(
        processed.config.statement,
        Statement::Config(vec![
            Node {
                file: "tokens.cpp".to_string(),
                start: (0, (1, 1)),
                end: (18, (1, 19)),
                line: "#define NAME Brett".to_string(),
                statement: Statement::Define {
                    ident: "NAME".to_string(),
                    value: Box::new(Node {
                        file: "tokens.cpp".to_string(),
                        start: (13, (1, 14)),
                        end: (18, (1, 19)),
                        line: "Brett".to_string(),
                        statement: Statement::Unquoted(vec![
                            Node {
                                file: "tokens.cpp".to_string(),
                                start: (13, (1, 14)),
                                end: (14, (1, 15)),
                                line: "B".to_string(),
                                statement: Statement::Char('B')
                            },
                            Node {
                                file: "tokens.cpp".to_string(),
                                start: (14, (1, 15)),
                                end: (15, (1, 16)),
                                line: "r".to_string(),
                                statement: Statement::Char('r')
                            },
                            Node {
                                file: "tokens.cpp".to_string(),
                                start: (15, (1, 16)),
                                end: (16, (1, 17)),
                                line: "e".to_string(),
                                statement: Statement::Char('e')
                            },
                            Node {
                                file: "tokens.cpp".to_string(),
                                start: (16, (1, 17)),
                                end: (17, (1, 18)),
                                line: "t".to_string(),
                                statement: Statement::Char('t')
                            },
                            Node {
                                file: "tokens.cpp".to_string(),
                                start: (17, (1, 18)),
                                end: (18, (1, 19)),
                                line: "t".to_string(),
                                statement: Statement::Char('t')
                            }
                        ])
                    })
                }
            },
            Node {
                file: "tokens.cpp".to_string(),
                start: (19, (2, 1)),
                end: (39, (3, 1)),
                line: "#define QUOTE(s) #s\n".to_string(),
                statement: Statement::DefineMacro {
                    ident: "QUOTE".to_string(),
                    args: vec!["s".to_string()],
                    value: Box::new(Node {
                        file: "tokens.cpp".to_string(),
                        start: (36, (2, 18)),
                        end: (39, (3, 1)),
                        line: "#s\n".to_string(),
                        statement: Statement::MacroBody("#s".to_string())
                    })
                }
            },
            Node {
                file: "tokens.cpp".to_string(),
                start: (39, (3, 1)),
                end: (64, (4, 1)),
                line: "#define APPEND(a,b) a##b\n".to_string(),
                statement: Statement::DefineMacro {
                    ident: "APPEND".to_string(),
                    args: vec!["a".to_string(), "b".to_string()],
                    value: Box::new(Node {
                        file: "tokens.cpp".to_string(),
                        start: (59, (3, 21)),
                        end: (64, (4, 1)),
                        line: "a##b\n".to_string(),
                        statement: Statement::MacroBody("a##b".to_string())
                    })
                }
            },
            Node {
                file: "tokens.cpp".to_string(),
                start: (65, (5, 1)),
                end: (83, (5, 19)),
                line: "name = QUOTE(NAME)".to_string(),
                statement: Statement::Property {
                    ident: Box::new(Node {
                        file: "tokens.cpp".to_string(),
                        start: (65, (5, 1)),
                        end: (69, (5, 5)),
                        line: "name".to_string(),
                        statement: Statement::Ident("name".to_string())
                    }),
                    value: Box::new(Node {
                        file: "tokens.cpp".to_string(),
                        start: (72, (5, 8)),
                        end: (83, (5, 19)),
                        line: "QUOTE(NAME)".to_string(),
                        statement: Statement::InternalStr("Brett".to_string())
                    }),
                    expand: false
                }
            },
            Node {
                file: "tokens.cpp".to_string(),
                start: (85, (6, 1)),
                end: (124, (6, 40)),
                line: "tagged_name = QUOTE(APPEND(USER_,NAME))".to_string(),
                statement: Statement::Property {
                    ident: Box::new(Node {
                        file: "tokens.cpp".to_string(),
                        start: (85, (6, 1)),
                        end: (96, (6, 12)),
                        line: "tagged_name".to_string(),
                        statement: Statement::Ident("tagged_name".to_string())
                    }),
                    value: Box::new(Node {
                        file: "tokens.cpp".to_string(),
                        start: (99, (6, 15)),
                        end: (124, (6, 40)),
                        line: "QUOTE(APPEND(USER_,NAME))".to_string(),
                        statement: Statement::InternalStr("USER_Brett".to_string())
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
    use std::io::Read;
    let mut test_against = Vec::new();
    std::fs::File::open("tests/tokens.bin")
        .unwrap()
        .read_to_end(&mut test_against)
        .unwrap();
    assert_eq!(rapified.get_ref(), &test_against);
}
