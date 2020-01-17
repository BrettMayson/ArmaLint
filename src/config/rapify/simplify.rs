use super::super::{get_ident, Node, Statement, AST};
use super::{Array, ArrayElement, Class, Config, Entry};
use crate::strum::AsStaticRef;
use crate::ArmaLintError;

impl Into<ArrayElement> for Entry {
    fn into(self) -> ArrayElement {
        match self {
            Entry::Str(v) => ArrayElement::Str(v),
            Entry::Float(v) => ArrayElement::Float(v),
            Entry::Int(v) => ArrayElement::Int(v),
            Entry::Array(v) => ArrayElement::Array(v),
            _ => panic!("Invalid item was found in array: {:?}", self),
        }
    }
}

impl Config {
    pub fn from_ast(ast: AST) -> Result<Self, ArmaLintError> {
        if !ast.processed {
            return Err(ArmaLintError::NotProcessed);
        }
        if let Statement::Config(inner) = ast.config.statement {
            Ok(Config {
                root: Class {
                    parent: String::new(),
                    external: false,
                    deletion: false,
                    entries: get_entries(inner)?,
                },
            })
        } else {
            Err(ArmaLintError::NotRoot)
        }
    }
}

pub fn get_entries(nodes: Vec<Node>) -> Result<Vec<(String, Entry)>, ArmaLintError> {
    let mut entries = Vec::new();
    for node in nodes {
        if let Some((ident, entry)) = get_entry(node)? {
            entries.push((ident, entry));
        }
    }
    Ok(entries)
}

pub fn get_entry(node: Node) -> Result<Option<(String, Entry)>, ArmaLintError> {
    Ok(match node.statement {
        Statement::Class { ident, extends, props } => Some((
            get_ident(ident.statement)?,
            Entry::Class(Class {
                parent: {
                    if let Some(ex) = extends {
                        get_ident(ex.statement)?
                    } else {
                        String::new()
                    }
                },
                deletion: false,
                external: false,
                entries: get_entries(props)?,
            }),
        )),
        Statement::ClassDef(ident) => Some((
            get_ident(ident.statement)?,
            Entry::Class(Class {
                parent: String::new(),
                deletion: false,
                external: true,
                entries: Vec::new(),
            }),
        )),
        Statement::Property { ident, value, expand } => {
            Some((get_ident(ident.statement)?, get_value(value.statement, expand)?))
        }
        // Ignore
        _ => {
            panic!("Not ready for {:#?}", node);
        }
    })
}

pub fn get_value(statement: Statement, expand: bool) -> Result<Entry, ArmaLintError> {
    Ok(match statement {
        Statement::Integer(val) => Entry::Int(val),
        Statement::Float(val) => Entry::Float(val),
        Statement::Str(val) => Entry::Str(val),
        Statement::InternalStr(val) => Entry::Str(val),
        Statement::Processed(val, _) => get_value(*val, expand)?,
        Statement::Array(val) => Entry::Array(Array {
            expand,
            elements: get_array(val)?,
        }),
        _ => {
            return Err(ArmaLintError::InvalidProperty(format!(
                "Invalid property type `{}`",
                statement.as_static()
            )))
        }
    })
}

pub fn get_array(nodes: Vec<Node>) -> Result<Vec<ArrayElement>, ArmaLintError> {
    let mut elements = Vec::new();
    for n in nodes {
        let expand = if let Statement::Property { expand: e, .. } = n.statement {
            e
        } else {
            false
        };
        elements.push(get_value(n.statement, expand)?.into());
    }
    Ok(elements)
}
