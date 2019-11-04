use armalint;
use armalint::config::{Node, Statement};

const FILE: &str = "tests/basic.cpp";
const FILENAME: &str = "basic.cpp";

fn import(filename: &str) -> String {
    println!("Including {}", filename);
    std::fs::read_to_string(filename).unwrap()
}

#[test]
fn parse() {
    let content = std::fs::read_to_string(FILE).unwrap();
    let ast = armalint::config::parse(FILENAME, &content, import).unwrap();
    use armalint::config::{Node, Statement};
    assert_eq!(
        ast.config.statement,
        Statement::Config(vec![
            Node {
                file: FILENAME.to_string(),
                start: (1, 1),
                end: (1, 11),
                statement: Statement::ClassDef(Box::new(Node {
                    file: FILENAME.to_string(),
                    start: (1, 7),
                    end: (1, 11),
                    statement: Statement::Ident(String::from("Test"))
                }))
            },
            Node {
                file: FILENAME.to_string(),
                start: (2, 1),
                end: (7, 2),
                statement: Statement::Class {
                    ident: Box::new(Node {
                        file: FILENAME.to_string(),
                        start: (2, 7),
                        end: (2, 16),
                        statement: Statement::Ident(String::from("TestClass"))
                    }),
                    extends: Some(Box::new(Node {
                        file: FILENAME.to_string(),
                        start: (2, 18),
                        end: (2, 22),
                        statement: Statement::Ident(String::from("Test"))
                    })),
                    props: vec![
                        Node {
                            file: FILENAME.to_string(),
                            start: (3, 5),
                            end: (3, 24),
                            statement: Statement::Property {
                                ident: Box::new(Node {
                                    file: FILENAME.to_string(),
                                    start: (3, 5),
                                    end: (3, 12),
                                    statement: Statement::IdentArray(String::from("array"))
                                }),
                                value: Box::new(Node {
                                    file: FILENAME.to_string(),
                                    start: (3, 15),
                                    end: (3, 24),
                                    statement: Statement::Array(vec![
                                        Node {
                                            file: FILENAME.to_string(),
                                            start: (3, 16),
                                            end: (3, 17),
                                            statement: Statement::Integer(1)
                                        },
                                        Node {
                                            file: FILENAME.to_string(),
                                            start: (3, 19),
                                            end: (3, 20),
                                            statement: Statement::Integer(3)
                                        },
                                        Node {
                                            file: FILENAME.to_string(),
                                            start: (3, 22),
                                            end: (3, 23),
                                            statement: Statement::Integer(5)
                                        }
                                    ])
                                }),
                                expand: false
                            }
                        },
                        Node {
                            file: FILENAME.to_string(),
                            start: (4, 5),
                            end: (4, 35),
                            statement: Statement::Property {
                                ident: Box::new(Node {
                                    file: FILENAME.to_string(),
                                    start: (4, 5),
                                    end: (4, 16),
                                    statement: Statement::IdentArray(String::from("deepArray"))
                                }),
                                value: Box::new(Node {
                                    file: FILENAME.to_string(),
                                    start: (4, 19),
                                    end: (4, 35),
                                    statement: Statement::Array(vec![
                                        Node {
                                            file: FILENAME.to_string(),
                                            start: (4, 20),
                                            end: (4, 26),
                                            statement: Statement::Array(vec![
                                                Node {
                                                    file: FILENAME.to_string(),
                                                    start: (4, 21),
                                                    end: (4, 22),
                                                    statement: Statement::Integer(1)
                                                },
                                                Node {
                                                    file: FILENAME.to_string(),
                                                    start: (4, 24),
                                                    end: (4, 25),
                                                    statement: Statement::Integer(2)
                                                }
                                            ])
                                        },
                                        Node {
                                            file: FILENAME.to_string(),
                                            start: (4, 28),
                                            end: (4, 34),
                                            statement: Statement::Array(vec![
                                                Node {
                                                    file: FILENAME.to_string(),
                                                    start: (4, 29),
                                                    end: (4, 30),
                                                    statement: Statement::Integer(3)
                                                },
                                                Node {
                                                    file: FILENAME.to_string(),
                                                    start: (4, 32),
                                                    end: (4, 33),
                                                    statement: Statement::Integer(4)
                                                }
                                            ])
                                        }
                                    ])
                                }),
                                expand: false
                            }
                        },
                        Node {
                            file: FILENAME.to_string(),
                            start: (5, 5),
                            end: (5, 39),
                            statement: Statement::Property {
                                ident: Box::new(Node {
                                    file: FILENAME.to_string(),
                                    start: (5, 5),
                                    end: (5, 15),
                                    statement: Statement::Ident(String::from("someString"))
                                }),
                                value: Box::new(Node {
                                    file: FILENAME.to_string(),
                                    start: (5, 19),
                                    end: (5, 38),
                                    statement: Statement::Str(String::from("This is some string"))
                                }),
                                expand: false
                            }
                        },
                        Node {
                            file: FILENAME.to_string(),
                            start: (6, 5),
                            end: (6, 21),
                            statement: Statement::Property {
                                ident: Box::new(Node {
                                    file: FILENAME.to_string(),
                                    start: (6, 5),
                                    end: (6, 15),
                                    statement: Statement::Ident(String::from("someNumber"))
                                }),
                                value: Box::new(Node {
                                    file: FILENAME.to_string(),
                                    start: (6, 18),
                                    end: (6, 21),
                                    statement: Statement::Integer(123)
                                }),
                                expand: false
                            }
                        }
                    ]
                }
            }
        ])
    );
}

#[test]
fn preprocess() {
    let content = std::fs::read_to_string(FILE).unwrap();
    let ast = armalint::config::parse(FILENAME, &content, |_| panic!("no import")).unwrap();
    let mut preprocessor = armalint::config::PreProcessor::new();
    let processed_ast = preprocessor.process(ast).unwrap();
    assert_eq!(
        processed_ast.config.statement,
        Statement::Config(vec![
            Node {
                file: FILENAME.to_string(),
                start: (1, 1),
                end: (1, 11),
                statement: Statement::ClassDef(Box::new(Node {
                    file: FILENAME.to_string(),
                    start: (1, 7),
                    end: (1, 11),
                    statement: Statement::Ident(String::from("Test"))
                }))
            },
            Node {
                file: FILENAME.to_string(),
                start: (2, 1),
                end: (7, 2),
                statement: Statement::Class {
                    ident: Box::new(Node {
                        file: FILENAME.to_string(),
                        start: (2, 7),
                        end: (2, 16),
                        statement: Statement::Ident(String::from("TestClass"))
                    }),
                    extends: Some(Box::new(Node {
                        file: FILENAME.to_string(),
                        start: (2, 18),
                        end: (2, 22),
                        statement: Statement::Ident(String::from("Test"))
                    })),
                    props: vec![
                        Node {
                            file: FILENAME.to_string(),
                            start: (3, 5),
                            end: (3, 24),
                            statement: Statement::Property {
                                ident: Box::new(Node {
                                    file: FILENAME.to_string(),
                                    start: (3, 5),
                                    end: (3, 12),
                                    statement: Statement::IdentArray(String::from("array"))
                                }),
                                value: Box::new(Node {
                                    file: FILENAME.to_string(),
                                    start: (3, 15),
                                    end: (3, 24),
                                    statement: Statement::Array(vec![
                                        Node {
                                            file: FILENAME.to_string(),
                                            start: (3, 16),
                                            end: (3, 17),
                                            statement: Statement::Integer(1)
                                        },
                                        Node {
                                            file: FILENAME.to_string(),
                                            start: (3, 19),
                                            end: (3, 20),
                                            statement: Statement::Integer(3)
                                        },
                                        Node {
                                            file: FILENAME.to_string(),
                                            start: (3, 22),
                                            end: (3, 23),
                                            statement: Statement::Integer(5)
                                        }
                                    ])
                                }),
                                expand: false
                            }
                        },
                        Node {
                            file: FILENAME.to_string(),
                            start: (4, 5),
                            end: (4, 35),
                            statement: Statement::Property {
                                ident: Box::new(Node {
                                    file: FILENAME.to_string(),
                                    start: (4, 5),
                                    end: (4, 16),
                                    statement: Statement::IdentArray(String::from("deepArray"))
                                }),
                                value: Box::new(Node {
                                    file: FILENAME.to_string(),
                                    start: (4, 19),
                                    end: (4, 35),
                                    statement: Statement::Array(vec![
                                        Node {
                                            file: FILENAME.to_string(),
                                            start: (4, 20),
                                            end: (4, 26),
                                            statement: Statement::Array(vec![
                                                Node {
                                                    file: FILENAME.to_string(),
                                                    start: (4, 21),
                                                    end: (4, 22),
                                                    statement: Statement::Integer(1)
                                                },
                                                Node {
                                                    file: FILENAME.to_string(),
                                                    start: (4, 24),
                                                    end: (4, 25),
                                                    statement: Statement::Integer(2)
                                                }
                                            ])
                                        },
                                        Node {
                                            file: FILENAME.to_string(),
                                            start: (4, 28),
                                            end: (4, 34),
                                            statement: Statement::Array(vec![
                                                Node {
                                                    file: FILENAME.to_string(),
                                                    start: (4, 29),
                                                    end: (4, 30),
                                                    statement: Statement::Integer(3)
                                                },
                                                Node {
                                                    file: FILENAME.to_string(),
                                                    start: (4, 32),
                                                    end: (4, 33),
                                                    statement: Statement::Integer(4)
                                                }
                                            ])
                                        }
                                    ])
                                }),
                                expand: false
                            }
                        },
                        Node {
                            file: FILENAME.to_string(),
                            start: (5, 5),
                            end: (5, 39),
                            statement: Statement::Property {
                                ident: Box::new(Node {
                                    file: FILENAME.to_string(),
                                    start: (5, 5),
                                    end: (5, 15),
                                    statement: Statement::Ident(String::from("someString"))
                                }),
                                value: Box::new(Node {
                                    file: FILENAME.to_string(),
                                    start: (5, 19),
                                    end: (5, 38),
                                    statement: Statement::Str(String::from("This is some string"))
                                }),
                                expand: false
                            }
                        },
                        Node {
                            file: FILENAME.to_string(),
                            start: (6, 5),
                            end: (6, 21),
                            statement: Statement::Property {
                                ident: Box::new(Node {
                                    file: FILENAME.to_string(),
                                    start: (6, 5),
                                    end: (6, 15),
                                    statement: Statement::Ident(String::from("someNumber"))
                                }),
                                value: Box::new(Node {
                                    file: FILENAME.to_string(),
                                    start: (6, 18),
                                    end: (6, 21),
                                    statement: Statement::Integer(123)
                                }),
                                expand: false
                            }
                        }
                    ]
                }
            }
        ])
    );
}

#[test]
fn simplify() {
    let content = std::fs::read_to_string(FILE).unwrap();
    let ast = armalint::config::parse(FILENAME, &content, |_| panic!("no import")).unwrap();
    let mut preprocessor = armalint::config::PreProcessor::new();
    let processed = preprocessor.process(ast).unwrap();
    armalint::config::simplify::Config::from_ast(processed).unwrap();
}

#[test]
fn rapify() {
    let content = std::fs::read_to_string(FILE).unwrap();
    let ast = armalint::config::parse(FILENAME, &content, |_| panic!("no import")).unwrap();
    let mut preprocessor = armalint::config::PreProcessor::new();
    let processed = preprocessor.process(ast).unwrap();
    let simple = armalint::config::simplify::Config::from_ast(processed).unwrap();
    let mut rapified = std::io::Cursor::new(Vec::new());
    simple.write_rapified(&mut rapified).unwrap();
    use std::io::Read;
    let mut test_against = Vec::new();
    std::fs::File::open("tests/basic.bin")
        .unwrap()
        .read_to_end(&mut test_against)
        .unwrap();
    assert_eq!(rapified.get_ref(), &test_against);
}
