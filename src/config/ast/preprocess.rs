use std::collections::HashMap;

use super::{Node, Renderer, Statement, AST};
use crate::ArmaLintError;

type ResultNodeVec = Result<Vec<Node>, ArmaLintError>;

#[derive(Default)]
pub struct PreProcessor {
    defines: HashMap<String, Node>,
    macros: HashMap<String, (Vec<String>, Node)>,
    valid: bool,
}
impl PreProcessor {
    pub fn new() -> Self {
        Self {
            defines: HashMap::new(),
            macros: HashMap::new(),
            valid: true,
        }
    }

    pub fn report(&self) {
        println!("=== Defines\n{:?}", self.defines);
        println!("=== Macros\n{:?}", self.macros);
        println!("=== Valid: {}", self.valid);
    }

    pub fn process(&mut self, ast: AST) -> Result<AST, ArmaLintError> {
        let mut ast = ast.clone();
        let config = match ast.config.statement {
            Statement::Config(c) => c,
            _ => return Err(ArmaLintError::PreprocessNotRoot),
        };
        ast.config.statement = Statement::Config(self.process_nodes(config)?);
        Ok(ast)
    }

    pub fn process_nodes(&mut self, nodes: Vec<Node>) -> ResultNodeVec {
        Ok(nodes
            .into_iter()
            .map(|x| self.process_node(x))
            .collect::<Result<Vec<Node>, ArmaLintError>>()?)
    }

    pub fn process_node(&mut self, node: Node) -> Result<Node, ArmaLintError> {
        let mut node = node.clone();
        match &mut node.statement {
            Statement::Property { ident, value } => {
                node.statement = Statement::Property {
                    ident: Box::new(self.process_node(*ident.clone())?),
                    value: Box::new(self.process_node(*value.clone())?),
                };
            }
            Statement::Class {
                ident,
                extends,
                props,
            } => {
                node.statement = Statement::Class {
                    ident: Box::new(self.process_node(*ident.clone())?),
                    extends: if let Some(e) = extends {
                        Some(Box::new(self.process_node(*e.clone())?))
                    } else {
                        None
                    },
                    props: self.process_nodes(props.to_vec())?,
                }
            }
            Statement::Array(values) => {
                node.statement = Statement::Array(
                    values
                        .iter()
                        .map(|x| self.process_node(x.clone()))
                        .collect::<ResultNodeVec>()?,
                );
            }
            Statement::Bool(_) => {}
            Statement::Integer(_) => {}
            Statement::Str(_) => {}
            Statement::FILE => {
                node.statement = Statement::Processed(
                    Box::new(Statement::InternalStr(node.file.clone())),
                    Box::new(node.statement),
                );
            }
            Statement::LINE => {
                node.statement = Statement::Processed(
                    Box::new(Statement::Integer(node.start.0 as i32)),
                    Box::new(node.statement),
                );
            }
            Statement::Ident(val) => {
                if let Some(s) = self.defines.get(val) {
                    node.statement = Statement::Defined(Box::new(s.clone()));
                }
            }
            Statement::IdentArray(val) => {
                if let Some(s) = self.defines.get(val) {
                    node.statement = Statement::Defined(Box::new(s.clone()));
                }
            }
            Statement::ClassDef(ident) => {
                node.statement = Statement::ClassDef(Box::new(self.process_node(*ident.clone())?));
            }
            Statement::Config(nodes) => {
                node.statement = Statement::Config(self.process_nodes(nodes.to_vec())?);
            }
            Statement::EOI => {}
            // Directives
            Statement::Define { ident, value } => {
                self.defines.remove(ident);
                self.macros.remove(ident);
                let data = self.process_node(*value.clone())?;
                self.defines.insert(ident.to_string(), data);
            }
            Statement::DefineMacro { ident, args, value } => {
                self.defines.remove(ident);
                self.macros.remove(ident);
                self.macros
                    .insert(ident.to_string(), (args.to_vec(), *value.clone()));
            }
            Statement::MacroCall { ident, args } => {
                if let Some(mac) = self.macros.get(ident) {
                    let (mac_args, mac_node) = mac.clone();
                    if mac_args.len() != args.len() {
                        node.statement = Statement::InvalidCall(
                            format!(
                                "Calling macro `{}` with `{}` args, requires `{}`",
                                ident,
                                args.len(),
                                mac_args.len()
                            ),
                            Box::new(node.statement.clone()),
                        );
                        self.valid = false;
                    } else {
                        let old_defines = self.defines.clone();
                        for (i, val) in args.iter().enumerate() {
                            let macro_body = self.process_node(val.clone())?;
                            self.defines
                                .insert(mac_args.get(i).unwrap().to_string(), macro_body);
                        }
                        node.statement = self.process_node(mac_node)?.statement;
                        self.defines = old_defines;
                    }
                } else {
                    node.statement = Statement::Undefined(
                        format!("Call to undefined macro `{}`", ident),
                        Box::new(node.statement.clone()),
                    );
                    self.valid = false;
                }
            }
            Statement::MacroCallArg(inner_args) => {
                let mut output = String::new();
                for inner_arg in inner_args {
                    match inner_arg.statement {
                        Statement::Char(c) => {
                            output.push(c);
                        }
                        _ => {
                            let val = self.process_node(inner_arg.clone())?;
                            match val.statement {
                                Statement::Processed(s, _) => {
                                    node.statement = *s;
                                    return Ok(node);
                                }
                                Statement::InternalStr(s) => {
                                    output.push_str(&s);
                                }
                                _ => {
                                    panic!("inner: {:#?}", val);
                                }
                            }
                        }
                    }
                }
                node.statement = Statement::Processed(
                    Box::new(if let Some(val) = self.defines.get(&output) {
                        Statement::Defined(Box::new(val.clone()))
                    } else {
                        Statement::InternalStr(self.tokens(output)?)
                    }),
                    Box::new(node.statement),
                );
            }
            Statement::MacroBody(s) => {
                node.statement = Statement::InternalStr(self.tokens(s.to_string())?);
            }
            Statement::Unquoted(children) => {
                let mut output = String::new();
                for child in children {
                    match child.statement {
                        Statement::Char(c) => {
                            output.push(c);
                        }
                        Statement::MacroCall { ident: _, args: _ } => {
                            match self.process_node(child.clone())?.statement {
                                Statement::InternalStr(s) => {
                                    output.push_str(&s);
                                }
                                _ => panic!("Unquoted.MacroCall needs to handle: {:#?}", child.statement),
                            }
                        }
                        _ => panic!("Unquoted needs to handle: {:#?}", child.statement),
                    }
                }

                node.statement = Statement::Processed(
                    Box::new(if let Some(val) = self.defines.get(&output) {
                        Statement::Defined(Box::new(val.clone()))
                    } else {
                        Statement::InternalStr(self.tokens(output)?)
                    }),
                    Box::new(node.statement),
                );
            }
            Statement::Undefine(ident) => {
                if self.macros.remove(ident).is_none() {
                    node.statement = Statement::Undefined(
                        format!("Attempt to undefine an undefined identifier `{}`", ident),
                        Box::new(node.statement.clone()),
                    );
                    self.valid = false;
                }
            }
            Statement::IfDef {
                ident,
                positive,
                negative,
            } => {
                node.statement =
                    if self.defines.contains_key(ident) || self.macros.contains_key(ident) {
                        Statement::Inserted(self.process_nodes(positive.to_vec())?)
                    } else if let Some(n) = negative {
                        Statement::Inserted(self.process_nodes(n.to_vec())?)
                    } else {
                        Statement::Gone
                    };
            }
            _ => {
                println!("No method for {:?}", node);
                unimplemented!()
            }
        }
        Ok(node)
    }

    pub fn tokens(&self, text: String) -> Result<String, ArmaLintError> {
        let s = text.clone();
        let mut output = Vec::new();
        for token in s.trim().split(" ") {
            if token.starts_with("#") {
                let ident = remove_first(token).unwrap();
                let data = if let Some(v) = self.defines.get(ident) {
                    Renderer::render_node(v.clone())?
                } else {
                    ident.to_string()
                };
                output.push(format!("\"{}\"", data));
            } else {
                output.push(if let Some(v) = self.defines.get(token) {
                    Renderer::render_node(v.clone())?
                } else {
                    token.to_string()
                });
            }
        }
        Ok(output.join(" "))
    }
}

fn remove_first(s: &str) -> Option<&str> {
    s.chars().next().map(|c| &s[c.len_utf8()..])
}
