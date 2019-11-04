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
/// Abstract Syntax Tree
pub struct AST {
    pub config: Node,
    pub processed: bool,
}

/// Converts a raw string into an AST
///
/// The resolver is used to find files for #include
/// ```rs
/// let content = "#include <myfile.hpp>";
///
/// armalint::config::parse("config.cpp", content, |filename| {
///     std::fs::read_to_string(filename)
/// });
/// ```
pub fn parse(file: &str, source: &str) -> Result<AST, ArmaLintError> {
    if source.starts_with("#s") {
        return Err(ArmaLintError::NotRoot);
    }
    let clean = source.replace("\r", "");
    let pair = ConfigParser::parse(Rule::config, &clean)
        .unwrap()
        .next()
        .ok_or_else(|| ArmaLintError::InvalidInput(clean.clone()))?;
    Ok(AST {
        config: Node::from_expr(file, source, pair, |filename| std::fs::read_to_string(filename))?,
        processed: false,
    })
}

/// Use a custom resolver
///
/// The resolver is used to find files for #include
/// ```rs
/// let content = "#include <myfile.hpp>";
///
/// armalint::config::parse("config.cpp", content, |filename| {
///     std::fs::read_to_string(filename)
/// });
/// ```
pub fn parse_with_resolver<F>(file: &str, source: &str, resolver: F) -> Result<AST, ArmaLintError>
where
    F: Fn(&str) -> Result<String, std::io::Error> + Copy,
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
            pest::error::ErrorVariant::ParsingError { positives, negatives } => ArmaLintError::ParsingError {
                positives: positives.into_iter().map(|x| format!("{:?}", x)).collect(),
                negatives: negatives.into_iter().map(|x| format!("{:?}", x)).collect(),
            },
            pest::error::ErrorVariant::CustomError { message } => {
                panic!(message);
            }
        }
    }
}
