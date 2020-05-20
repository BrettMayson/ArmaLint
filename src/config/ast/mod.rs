mod node;
pub use node::Node;

mod statement;
pub use statement::Statement;

use std::collections::HashMap;

use super::Report;
use crate::ArmaLintError;

#[derive(Debug, Clone)]
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

    pub fn process(&self) -> Result<Self, ArmaLintError> {
        let ast = self.clone();
        let mut preprocessor = super::PreProcessor::new();
        preprocessor.process(ast)
    }
}
