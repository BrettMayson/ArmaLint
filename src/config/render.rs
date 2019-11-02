use super::{Node, Statement, AST};
use crate::ArmaLintError;

pub struct Renderer {}
impl Renderer {
    pub fn render(ast: AST) -> Result<String, ArmaLintError> {
        let mut output = String::new();
        let config = match ast.config.statement {
            Statement::Config(c) => c,
            _ => return Err(ArmaLintError::NotRoot),
        };
        output.push_str(&Renderer::render_nodes(config)?);
        Ok(output)
    }

    pub fn render_nodes(nodes: Vec<Node>) -> Result<String, ArmaLintError> {
        let mut output = String::new();
        for node in nodes {
            output.push_str(&Renderer::render_node(node)?);
        }
        Ok(output)
    }

    pub fn render_node(node: Node) -> Result<String, ArmaLintError> {
        let mut output = String::new();
        output.push_str(&Renderer::render_statement(node.statement)?);
        Ok(output)
    }

    pub fn render_statement(statement: Statement) -> Result<String, ArmaLintError> {
        let mut output = String::new();
        match statement {
            Statement::Property {
                ident,
                value,
                expand,
            } => {
                output.push_str(&format!(
                    "{} {} {};\n",
                    Renderer::render_node(*ident)?,
                    if expand { "+=" } else { "=" },
                    Renderer::render_statement(value.statement)?
                ));
            }
            Statement::Ident(val) => output.push_str(&val.to_string()),
            Statement::IdentArray(val) => output.push_str(&format!("{}[]", val.to_string())),
            Statement::Bool(val) => output.push_str(&val.to_string()),
            Statement::Str(val) => output.push_str(&format!("\"{}\"", val.replace('"', "\"\""))),
            Statement::Integer(val) => output.push_str(&val.to_string()),
            Statement::Float(val) => output.push_str(&val.to_string()),
            Statement::Char(val) => output.push(val),
            Statement::InternalStr(val) => output.push_str(&val.to_string()),
            //Statement::Unquoted(val) => output.push_str(&format!("\"{}\"", val.replace('"', "\"\""))),
            Statement::Class {
                ident,
                extends,
                props,
            } => {
                output.push_str(&format!("class {}", Renderer::render_node(*ident)?));
                if let Some(extended) = extends {
                    output.push_str(&format!(": {}", Renderer::render_node(*extended)?));
                }
                output.push_str(if props.is_empty() { " {" } else { " {\n" });
                output.push_str(&Renderer::render_nodes(props)?);
                output.push_str("};\n");
            }
            Statement::ClassDef(ident) => {
                output.push_str(&format!("class {};\n", Renderer::render_node(*ident)?))
            }
            Statement::ClassDelete(ident) => {
                output.push_str(&format!("delete {};\n", Renderer::render_node(*ident)?))
            }
            Statement::Config(nodes) => output.push_str(&Renderer::render_nodes(nodes)?),
            Statement::Array(nodes) => {
                output.push('{');
                output.push_str(
                    &nodes
                        .iter()
                        .map(|x| Renderer::render_statement(x.statement.clone()))
                        .collect::<Result<Vec<String>, ArmaLintError>>()?
                        .join(", "),
                );
                output.push('}');
            }
            Statement::Processed(stmt, _) => output.push_str(&Renderer::render_statement(*stmt)?),
            Statement::Defined(node, orig) => {
                output.push_str(&Renderer::render_node(*node.clone())?)
            }
            Statement::Inserted(nodes) => output.push_str(&Renderer::render_nodes(nodes)?),
            // Should be processed out
            Statement::Unquoted(nodes) => output.push_str(&Renderer::render_nodes(nodes)?),
            Statement::FILE => {
                panic!("A file marker was not processed out, this should be reported as a bug")
            }
            Statement::LINE => {
                panic!("A line marker was not processed out, this should be reported as a bug")
            }
            Statement::IfDef {
                ident: _,
                positive: _,
                negative: _,
            } => panic!("An IfDef marker was not processed out, this should be reported as a bug"),
            Statement::MacroBody(_) => {
                panic!("A MacroBody marker was not processed out, this should be reported as a bug")
            }
            Statement::MacroCallArg(_) => panic!(
                "A MacroCallArg marker was not processed out, this should be reported as a bug"
            ),
            // Ignored
            Statement::Define { ident: _, value: _ } => {}
            Statement::DefineMacro {
                ident: _,
                args: _,
                value: _,
            } => {}
            Statement::Undefine(_) => {}
            Statement::Undefined(_, _) => {}
            Statement::MacroCall { ident: _, args: _ } => {}
            Statement::InvalidCall(_, _) => {}
            Statement::Gone => {}
        }
        Ok(output)
    }
}
