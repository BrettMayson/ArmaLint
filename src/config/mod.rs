mod parser;
pub use parser::{parse, Node, Statement, AST};

mod preprocess;
pub use preprocess::PreProcessor;

mod render;
pub use render::Renderer;

pub mod rapify;
pub mod simplify;

fn get_ident(stmt: Statement) -> Result<String, crate::ArmaLintError> {
    Ok(match stmt {
        Statement::Ident(val) => val.to_string(),
        _ => panic!("get ident wasn't given ident"),
    })
}

// Tests

#[test]
fn basic_class() {
    let content = r###"class something {
    data = "this is data";
    numbers[] = {1, 2, {3, 4}, 5};
    digit = 149;
    dec = 12.42;
};"###;
    parse("basic.cpp", content, |_x| panic!("No import")).unwrap();
}

#[test]
fn basic_statement_ast() {
    let content = r###"something = true;"###;
    let ast = parse("basic.cpp", content, |_x| panic!("No Import")).unwrap();
    assert_eq!(
        ast.config.statement,
        Statement::Config(vec![Node {
            file: "basic.cpp".to_string(),
            start: (1, 1),
            end: (1, 17),
            statement: Statement::Property {
                ident: Box::new(Node {
                    file: "basic.cpp".to_string(),
                    start: (1, 1),
                    end: (1, 10),
                    statement: Statement::Ident("something".to_string())
                }),
                value: Box::new(Node {
                    file: "basic.cpp".to_string(),
                    start: (1, 13),
                    end: (1, 17),
                    statement: Statement::Bool(true)
                }),
                expand: false,
            },
        }])
    );
}
