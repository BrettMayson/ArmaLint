use super::{Node, Statement, AST};
use crate::ArmaLintError;

mod options;
pub use options::{BracketStyle, IndentationType, RenderOptions};

/// Renders processed AST or simplified configs
#[derive(Clone, Copy, Hash)]
pub struct Renderer {
    options: RenderOptions,
}

impl Renderer {
    /// Create a new Renderer with the passed options
    pub fn new(options: RenderOptions) -> Self {
        Self { options }
    }

    /// Render the supplied AST
    pub fn render(self, ast: AST) -> Result<String, ArmaLintError> {
        let mut output = String::new();
        let config = match ast.config.statement {
            Statement::Config(c) => c,
            _ => return Err(ArmaLintError::NotRoot),
        };
        output.push_str(&self.render_nodes(config, 0)?);
        Ok(output.trim().to_string())
    }

    pub fn render_nodes(self, nodes: Vec<Node>, indent: u8) -> Result<String, ArmaLintError> {
        let mut output = String::new();
        for node in nodes {
            output.push_str(&self.render_node(node, indent)?);
        }
        Ok(output)
    }

    pub fn render_node(self, node: Node, indent: u8) -> Result<String, ArmaLintError> {
        let mut output = String::new();
        output.push_str(&self.render_statement(node.statement, indent)?);
        Ok(output)
    }

    pub fn render_statement(self, statement: Statement, indent: u8) -> Result<String, ArmaLintError> {
        let mut output = String::new();
        match statement {
            Statement::Property { ident, value, expand } => {
                output.push_str(&self.indent(indent));
                output.push_str(&format!(
                    "{} {} {};\n",
                    self.render_node(*ident, indent)?,
                    if expand { "+=" } else { "=" },
                    self.render_statement(value.statement, indent)?
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
            Statement::Quoted(val) => output.push_str(&format!("\"{}\"", &self.render_statement(*val, indent)?)),
            Statement::Class { ident, extends, props } => {
                output.push_str(&self.indent(indent));
                output.push_str(&format!("class {}", self.render_node(*ident, indent)?));
                if let Some(extended) = extends {
                    output.push_str(&format!(": {}", self.render_node(*extended, indent)?));
                }
                match self.options.bracket_style {
                    BracketStyle::Allman => {
                        output.push_str("\n");
                        output.push_str(&self.indent(indent));
                    }
                    BracketStyle::Linux => output.push_str(" "),
                }
                output.push_str(if props.is_empty() { "{" } else { "{\n" });
                output.push_str(&self.render_nodes(props, indent + 1)?);
                output.push_str(&self.indent(indent));
                output.push_str("};\n");
            }
            Statement::ClassDef(ident) => output.push_str(&format!("class {};\n", self.render_node(*ident, indent)?)),
            Statement::ClassDelete(ident) => output.push_str(&format!("delete {};\n", self.render_node(*ident, indent)?)),
            Statement::Config(nodes) => output.push_str(&self.render_nodes(nodes, indent)?),
            Statement::Array(nodes) => {
                output.push('{');
                output.push_str(
                    &nodes
                        .iter()
                        .map(|x| self.render_statement(x.statement.clone(), indent))
                        .collect::<Result<Vec<String>, ArmaLintError>>()?
                        .join(", "),
                );
                output.push('}');
            }
            Statement::Processed(stmt, _) => output.push_str(&self.render_statement(*stmt, indent)?),
            Statement::Defined(node, _) => output.push_str(&self.render_node(*node.clone(), indent)?),
            Statement::Inserted(nodes) => output.push_str(&self.render_nodes(nodes, indent)?),
            // Should be processed out
            Statement::Unquoted(nodes) => output.push_str(&self.render_nodes(nodes, indent)?),
            Statement::FILE => panic!("A file marker was not processed out, this should be reported as a bug"),
            Statement::LINE => panic!("A line marker was not processed out, this should be reported as a bug"),
            Statement::IfDef { .. } => panic!("An IfDef marker was not processed out, this should be reported as a bug"),
            Statement::IfNDef { .. } => panic!("An IfNDef marker was not processed out, this should be reported as a bug"),
            // Statement::MacroBody(_) => panic!("A MacroBody marker was not processed out, this should be reported as a bug"),
            Statement::MacroBody(_) => {}
            Statement::MacroCallArg(_) => {
                panic!("A MacroCallArg marker was not processed out, this should be reported as a bug")
            }
            // Ignored
            Statement::Define { .. } => {}
            Statement::DefineMacro { .. } => {}
            Statement::FlagAsIdent(_, _, _) => {}
            Statement::Gone => {}
            Statement::InvalidCall(_, _, _) => {}
            Statement::MacroCall { .. } => {}
            Statement::Undefine(_) => {}
            Statement::Undefined(_, _) => {}
            // Warnings & Errors
            Statement::NonUppercaseDefine(_) => {}
            Statement::Redefine(_, _, _) => {}
        }
        Ok(output)
    }

    fn indent(self, indent: u8) -> String {
        repeat!(
            match self.options.indentation_type {
                IndentationType::Tab => String::from("\t"),
                IndentationType::Spaces(u) => repeat!(" ", u as usize),
                IndentationType::None => String::new(),
            },
            indent as usize
        )
    }
}

impl Default for Renderer {
    fn default() -> Self {
        Self {
            options: RenderOptions::default(),
        }
    }
}
