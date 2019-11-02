use pest::Parser;

mod node;
pub use node::Node;

mod preprocess;
pub use preprocess::PreProcessor;

mod render;
pub use render::Renderer;

mod statement;
pub use statement::Statement;

pub mod simplify;

use crate::ArmaLintError;

#[derive(Parser)]
#[grammar = "config/config.pest"]
pub struct ConfigParser;

#[derive(Debug, Clone)]
pub struct AST {
    pub config: Node,
    pub processed: bool,
}

pub fn parse<F>(file: &str, source: &str, resolver: F) -> Result<AST, ArmaLintError>
where
    F: Fn(&str) -> String + Copy,
{
    if source.starts_with("#s") {
        return Err(ArmaLintError::NotRoot);
    }
    let clean = source.replace("\r", "");
    let pair = ConfigParser::parse(Rule::config, &clean)
        .unwrap()
        .next()
        .ok_or_else(|| ArmaLintError::InvalidInput(clean.clone()))?;
    Ok(AST {
        config: Node::from_expr(file, source, pair, resolver)?,
        processed: false,
    })
}

// Error handling

impl From<pest::error::Error<Rule>> for ArmaLintError {
    fn from(err: pest::error::Error<Rule>) -> ArmaLintError {
        match err.variant {
            pest::error::ErrorVariant::ParsingError {
                positives,
                negatives,
            } => ArmaLintError::ParsingError {
                positives: positives.into_iter().map(|x| format!("{:?}", x)).collect(),
                negatives: negatives.into_iter().map(|x| format!("{:?}", x)).collect(),
            },
            pest::error::ErrorVariant::CustomError { message } => {
                panic!(message);
            }
        }
    }
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
    parse("basic.cpp", content, |x| panic!("No import")).unwrap();
}

#[test]
fn basic_statement_ast() {
    let content = r###"something = true;"###;
    let ast = parse("basic.cpp", content, |x| panic!("No Import")).unwrap();
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
