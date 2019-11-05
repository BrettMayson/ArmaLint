mod parser;
pub use parser::{parse, parse_with_resolver, Node, Statement, AST};

mod preprocess;
pub use preprocess::PreProcessor;

pub mod render;
pub use render::{RenderOptions, Renderer};

pub mod rapify;
pub mod simplify;

fn get_ident(stmt: Statement) -> Result<String, crate::ArmaLintError> {
    Ok(match stmt {
        Statement::Ident(val) => val,
        Statement::IdentArray(val) => val,
        Statement::InternalStr(val) => val,
        Statement::Processed(val, _) => get_ident(*val)?,
        Statement::Defined(val, _) => get_ident(val.statement)?,
        _ => panic!("get ident wasn't given ident: {:#?}", stmt),
    })
}

// Tests

#[test]
fn basic_statement_ast() {
    let content = r###"something = true;"###;
    let ast = parse("basic.cpp", content).unwrap();
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
