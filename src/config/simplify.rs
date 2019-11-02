use super::{Node, Statement, AST};
use crate::ArmaLintError;

use strum::AsStaticRef;

#[derive(Debug)]
pub struct Config {
    pub root: Class,
}

#[derive(Debug, Clone)]
pub struct Class {
    pub parent: String,
    pub external: bool,
    pub deletion: bool,
    pub entries: Vec<(String, Entry)>,
}

#[derive(Debug, Clone)]
pub enum Entry {
    Str(String),
    Float(f32),
    Int(i32),
    Array(Array),
    Class(Class),
    Invisible(Vec<(String, Entry)>),
}

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

#[derive(Debug, Clone)]
pub struct Array {
    pub expand: bool,
    pub elements: Vec<ArrayElement>,
}

#[derive(Debug, Clone)]
pub enum ArrayElement {
    Str(String),
    Float(f32),
    Int(i32),
    Array(Array),
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
                    entries: { get_entries(inner)? },
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
        Statement::Class {
            ident,
            extends,
            props,
        } => Some((
            ident.to_string(),
            Entry::Class(Class {
                parent: {
                    if let Some(ex) = extends {
                        ex.to_string()
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
            ident.to_string(),
            Entry::Class(Class {
                parent: String::new(),
                deletion: false,
                external: true,
                entries: Vec::new(),
            }),
        )),
        Statement::Property {
            ident,
            value,
            expand,
        } => Some((ident.to_string(), get_value(value.statement, expand)?)),
        Statement::Config(inner) => Some((String::new(), Entry::Invisible(get_entries(inner)?))),
        // Ignore
        Statement::DefineMacro { .. } => None,
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
        Statement::Processed(val, _) => Entry::Str(val.to_string()),
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
