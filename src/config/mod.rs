mod parser;
pub use parser::{parse, parse_with_resolver, Node, Statement, AST};

mod preprocess;
pub use preprocess::process;

mod report;
pub use report::Report;

pub mod render;
pub use render::{RenderOptions, Renderer};

pub mod rapify;
pub mod simplify;

fn get_ident(stmt: Statement) -> Result<String, crate::ArmaLintError> {
    println!("given {:?}", stmt);
    Ok(match stmt {
        Statement::Ident(val) => val,
        Statement::IdentArray(val) => val,
        Statement::InternalStr(val) => val,
        Statement::Processed(val, _) => get_ident(*val)?,
        Statement::Defined(val, _) => get_ident(val.statement)?,
        Statement::MacroBody(val) => val,
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
            start: (0, (1, 1)),
            end: (16, (1, 17)),
            line: "something = true".to_string(),
            statement: Statement::Property {
                ident: Box::new(Node {
                    file: "basic.cpp".to_string(),
                    start: (0, (1, 1)),
                    end: (9, (1, 10)),
                    line: "something".to_string(),
                    statement: Statement::Ident("something".to_string())
                }),
                value: Box::new(Node {
                    file: "basic.cpp".to_string(),
                    start: (12, (1, 13)),
                    end: (16, (1, 17)),
                    line: "true".to_string(),
                    statement: Statement::Bool(true)
                }),
                expand: false
            }
        }])
    );
}
