use std::collections::HashMap;

use super::Report;
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
    pub files: HashMap<String, (Option<(String, usize)>, String)>,
    pub processed: bool,
    pub report: Option<Report>,
}

impl AST {
    pub fn valid(&self) -> bool {
        if let Some(report) = &self.report {
            report.errors.is_empty()
        } else {
            true
        }
    }
}

/// Converts a raw string into an AST
///
/// ```
/// let content = "value = 123;";
/// armalint::config::parse("config.cpp", content);
/// ```
pub fn parse(file: &str, source: &str) -> Result<AST, ArmaLintError> {
    if source.starts_with("#s") {
        return Err(ArmaLintError::NotRoot);
    }
    let clean = source.replace("\r", "");
    let mut files = HashMap::new();
    files.insert(file.to_string(), (None, clean.to_string()));
    let pair = ConfigParser::parse(Rule::file, &clean)?
        .next()
        .ok_or_else(|| ArmaLintError::InvalidInput(clean.clone()))?;
    let pair = pair.into_inner().next().unwrap();
    let (config, included) = Node::from_expr(file, source, pair, |filename| std::fs::read_to_string(filename))?;
    included.into_iter().for_each(|x| {
        files.insert(x.0, (x.1, x.2));
    });
    Ok(AST {
        config,
        files,
        processed: false,
        report: None,
    })
}

/// Use a custom resolver
///
/// The resolver is used to find files for #include
/// ```
/// let content = "#include <myfile.hpp>";
///
/// armalint::config::parse_with_resolver("config.cpp", content, |filename| {
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
    let mut files = HashMap::new();
    files.insert(file.to_string(), (None, clean.to_string()));
    let pair = ConfigParser::parse(Rule::file, &clean)?
        .next()
        .ok_or_else(|| ArmaLintError::InvalidInput(clean.clone()))?;
    let pair = pair.into_inner().next().unwrap();
    let (config, included) = Node::from_expr(file, source, pair, resolver)?;
    included.into_iter().for_each(|x| {
        files.insert(x.0, (x.1, x.2));
    });
    Ok(AST {
        config,
        files,
        processed: false,
        report: None,
    })
}

// Error handling

impl From<pest::error::Error<Rule>> for ArmaLintError {
    fn from(err: pest::error::Error<Rule>) -> ArmaLintError {
        match err.variant {
            pest::error::ErrorVariant::ParsingError { positives, negatives } => ArmaLintError::ParsingError {
                positives: positives.into_iter().map(|x| format!("{:?}", x)).collect(),
                negatives: negatives.into_iter().map(|x| format!("{:?}", x)).collect(),
                position: err.line_col,
            },
            pest::error::ErrorVariant::CustomError { message } => {
                panic!(message);
            }
        }
    }
}
