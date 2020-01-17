use armalint;
use armalint::config::{Node, Statement};

const FILE: &str = "tests/basic.cpp";
const FILENAME: &str = "basic.cpp";

#[test]
fn parse() {
    let content = std::fs::read_to_string(FILE).unwrap();
    let ast = armalint::config::parse(FILENAME, &content).unwrap();
    assert_eq!(
        ast.config.statement,
        Statement::Config(vec![
            Node {
                file: "basic.cpp".to_string(),
                start: (0, (1, 1)),
                end: (10, (1, 11)),
                line: "class Test".to_string(),
                statement: Statement::ClassDef(Box::new(Node {
                    file: "basic.cpp".to_string(),
                    start: (6, (1, 7)),
                    end: (10, (1, 11)),
                    line: "Test".to_string(),
                    statement: Statement::Ident("Test".to_string())
                }))
            },
            Node {
                file: "basic.cpp".to_string(),
                start: (12, (2, 1)),
                end: (182, (8, 2)),
                line: "class TestClass: Test {\n    array[] = {1, 3, 5};\n    deepArray[] = {{1, 2}, {3, 4}};\n    someString = \"This is some string\";\n    someNumber = 123;\n    someFloat = 3.14;\n}".to_string(),
                statement: Statement::Class {
                    ident: Box::new(Node {
                        file: "basic.cpp".to_string(),
                        start: (18, (2, 7)),
                        end: (27, (2, 16)),
                        line: "TestClass".to_string(),
                        statement: Statement::Ident("TestClass".to_string())
                    }),
                    extends: Some(Box::new(Node {
                        file: "basic.cpp".to_string(),
                        start: (29, (2, 18)),
                        end: (33, (2, 22)),
                        line: "Test".to_string(),
                        statement: Statement::Ident("Test".to_string())
                    })),
                    props: vec![
                        Node {
                            file: "basic.cpp".to_string(),
                            start: (40, (3, 5)),
                            end: (59, (3, 24)),
                            line: "array[] = {1, 3, 5}".to_string(),
                            statement: Statement::Property {
                                ident: Box::new(Node {
                                    file: "basic.cpp".to_string(),
                                    start: (40, (3, 5)),
                                    end: (47, (3, 12)),
                                    line: "array[]".to_string(),
                                    statement: Statement::IdentArray("array".to_string())
                                }),
                                value: Box::new(Node {
                                    file: "basic.cpp".to_string(),
                                    start: (50, (3, 15)),
                                    end: (59, (3, 24)),
                                    line: "{1, 3, 5}".to_string(),
                                    statement: Statement::Array(vec![
                                        Node {
                                            file: "basic.cpp".to_string(),
                                            start: (51, (3, 16)),
                                            end: (52, (3, 17)),
                                            line: "1".to_string(),
                                            statement: Statement::Integer(1)
                                        },
                                        Node {
                                            file: "basic.cpp".to_string(),
                                            start: (54, (3, 19)),
                                            end: (55, (3, 20)),
                                            line: "3".to_string(),
                                            statement: Statement::Integer(3)
                                        },
                                        Node {
                                            file: "basic.cpp".to_string(),
                                            start: (57, (3, 22)),
                                            end: (58, (3, 23)),
                                            line: "5".to_string(),
                                            statement: Statement::Integer(5)
                                        }
                                    ])
                                }),
                                expand: false
                            }
                        },
                        Node {
                            file: "basic.cpp".to_string(),
                            start: (65, (4, 5)),
                            end: (95, (4, 35)),
                            line: "deepArray[] = {{1, 2}, {3, 4}}".to_string(),
                            statement: Statement::Property {
                                ident: Box::new(Node {
                                    file: "basic.cpp".to_string(),
                                    start: (65, (4, 5)),
                                    end: (76, (4, 16)),
                                    line: "deepArray[]".to_string(),
                                    statement: Statement::IdentArray("deepArray".to_string())
                                }),
                                value: Box::new(Node {
                                    file: "basic.cpp".to_string(),
                                    start: (79, (4, 19)),
                                    end: (95, (4, 35)),
                                    line: "{{1, 2}, {3, 4}}".to_string(),
                                    statement: Statement::Array(vec![
                                        Node {
                                            file: "basic.cpp".to_string(),
                                            start: (80, (4, 20)),
                                            end: (86, (4, 26)),
                                            line: "{1, 2}".to_string(),
                                            statement: Statement::Array(vec![
                                                Node {
                                                    file: "basic.cpp".to_string(),
                                                    start: (81, (4, 21)),
                                                    end: (82, (4, 22)),
                                                    line: "1".to_string(),
                                                    statement: Statement::Integer(1)
                                                },
                                                Node {
                                                    file: "basic.cpp".to_string(),
                                                    start: (84, (4, 24)),
                                                    end: (85, (4, 25)),
                                                    line: "2".to_string(),
                                                    statement: Statement::Integer(2)
                                                }
                                            ])
                                        },
                                        Node {
                                            file: "basic.cpp".to_string(),
                                            start: (88, (4, 28)),
                                            end: (94, (4, 34)),
                                            line: "{3, 4}".to_string(),
                                            statement: Statement::Array(vec![
                                                Node {
                                                    file: "basic.cpp".to_string(),
                                                    start: (89, (4, 29)),
                                                    end: (90, (4, 30)),
                                                    line: "3".to_string(),
                                                    statement: Statement::Integer(3)
                                                },
                                                Node {
                                                    file: "basic.cpp".to_string(),
                                                    start: (92, (4, 32)),
                                                    end: (93, (4, 33)),
                                                    line: "4".to_string(),
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
                            file: "basic.cpp".to_string(),
                            start: (101, (5, 5)),
                            end: (135, (5, 39)),
                            line: "someString = \"This is some string\"".to_string(),
                            statement: Statement::Property {
                                ident: Box::new(Node {
                                    file: "basic.cpp".to_string(),
                                    start: (101, (5, 5)),
                                    end: (111, (5, 15)),
                                    line: "someString".to_string(),
                                    statement: Statement::Ident("someString".to_string())
                                }),
                                value: Box::new(Node {
                                    file: "basic.cpp".to_string(),
                                    start: (115, (5, 19)),
                                    end: (134, (5, 38)),
                                    line: "This is some string".to_string(),
                                    statement: Statement::Str("This is some string".to_string())
                                }),
                                expand: false
                            }
                        },
                        Node {
                            file: "basic.cpp".to_string(),
                            start: (141, (6, 5)),
                            end: (157, (6, 21)),
                            line: "someNumber = 123".to_string(),
                            statement: Statement::Property {
                                ident: Box::new(Node {
                                    file: "basic.cpp".to_string(),
                                    start: (141, (6, 5)),
                                    end: (151, (6, 15)),
                                    line: "someNumber".to_string(),
                                    statement: Statement::Ident("someNumber".to_string())
                                }),
                                value: Box::new(Node {
                                    file: "basic.cpp".to_string(),
                                    start: (154, (6, 18)),
                                    end: (157, (6, 21)),
                                    line: "123".to_string(),
                                    statement: Statement::Integer(123)
                                }),
                                expand: false
                            }
                        },
                        Node {
                            file: "basic.cpp".to_string(),
                            start: (163, (7, 5)),
                            end: (179, (7, 21)),
                            line: "someFloat = 3.14".to_string(),
                            statement: Statement::Property {
                                ident: Box::new(Node {
                                    file: "basic.cpp".to_string(),
                                    start: (163, (7, 5)),
                                    end: (172, (7, 14)),
                                    line: "someFloat".to_string(),
                                    statement: Statement::Ident("someFloat".to_string())
                                }),
                                value: Box::new(Node {
                                    file: "basic.cpp".to_string(),
                                    start: (175, (7, 17)),
                                    end: (179, (7, 21)),
                                    line: "3.14".to_string(),
                                    statement: Statement::Float(3.14)
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
    std::fs::File::open("tests/basic.bin")
        .unwrap()
        .read_to_end(&mut test_against)
        .unwrap();
    assert_eq!(rapified.get_ref(), &test_against);
}
