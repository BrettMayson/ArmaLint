use crate::ArmaLintError;

mod node;
pub use node::Node;

mod statement;
pub use statement::Statement;

use pest::Parser;

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
