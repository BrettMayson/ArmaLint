use super::{Statement, AST};
use crate::ArmaLintError;

mod actor;
use actor::Actor;

pub fn process(ast: AST) -> Result<AST, ArmaLintError> {
    let mut ast = ast.clone();
    let config = match ast.config.statement {
        Statement::Config(c) => c,
        _ => return Err(ArmaLintError::NotRoot),
    };
    let mut a = Actor::new();
    ast.config.statement = Statement::Config(a.process_nodes(config, None)?);
    ast.processed = true;
    ast.report = Some(a.report.clone());
    Ok(ast)
}
