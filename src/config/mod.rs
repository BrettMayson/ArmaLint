mod ast;
pub use ast::{Node, Statement, AST};

mod parser;
pub use parser::{parse, parse_with_resolver};
pub use parser::{ConfigParser, Rule};

mod preprocess;
pub use preprocess::PreProcessor;

mod report;
pub use report::Report;

mod render;
pub use render::{BracketStyle, RenderOptions, Renderer};

mod rapify;
pub mod simplify;

type ResultNodeVec = Result<Vec<Node>, crate::ArmaLintError>;

fn get_ident(stmt: Statement) -> Result<String, crate::ArmaLintError> {
    Ok(match stmt {
        Statement::Ident(val) => val,
        Statement::IdentArray(val) => val,
        Statement::InternalStr(val) => val,
        Statement::Processed(val, _) => get_ident(*val)?,
        Statement::Defined(val, _) => get_ident(val.statement)?,
        Statement::Inserted(ref nodes) => {
            let mut ret = String::new();
            for n in nodes {
                ret.push_str(&n.statement.string().unwrap());
            }
            ret
        }
        _ => panic!("get ident wasn't given ident: {:#?}", stmt),
    })
}
