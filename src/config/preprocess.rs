use std::collections::HashMap;

use super::{Node, Report, Statement, AST};
use crate::ArmaLintError;

type ResultNodeVec = Result<Vec<Node>, ArmaLintError>;

// TODO this should be private and constructed internally
#[derive(Default)]
pub struct PreProcessor {
    defines: HashMap<String, Node>,
    macros: HashMap<String, (Vec<String>, Node)>,
    valid: bool,
    report: Report,
}
impl PreProcessor {
    pub fn new() -> Self {
        Self {
            defines: HashMap::new(),
            macros: HashMap::new(),
            valid: true,
            report: Report::new(),
        }
    }

    pub fn process(&mut self, ast: AST) -> Result<(AST, Report), ArmaLintError> {
        let mut ast = ast.clone();
        let config = match ast.config.statement {
            Statement::Config(c) => c,
            _ => return Err(ArmaLintError::NotRoot),
        };
        ast.config.statement = Statement::Config(self.process_nodes(config, None)?);
        ast.processed = true;
        ast.valid = self.valid;
        Ok((ast, self.report.clone()))
    }

    pub fn process_nodes(&mut self, nodes: Vec<Node>, root_node: Option<Node>) -> ResultNodeVec {
        Ok(nodes
            .into_iter()
            .map(|x| self.process_node(x, root_node.clone()))
            .collect::<Result<Vec<Node>, ArmaLintError>>()?)
    }

    pub fn process_node(&mut self, node: Node, macro_root: Option<Node>) -> Result<Node, ArmaLintError> {
        let mut node = node.clone();
        let node_clone = node.clone();
        match &mut node.statement {
            Statement::Property { ident, value, expand } => {
                node.statement = Statement::Property {
                    ident: Box::new(self.process_node(*ident.clone(), macro_root.clone())?),
                    value: Box::new(self.process_node(*value.clone(), macro_root.clone())?),
                    expand: *expand,
                };
            }
            Statement::Class { ident, extends, props } => {
                node.statement = Statement::Class {
                    ident: Box::new(self.process_node(*ident.clone(), macro_root.clone())?),
                    extends: if let Some(e) = extends {
                        Some(Box::new(self.process_node(*e.clone(), macro_root.clone())?))
                    } else {
                        None
                    },
                    props: self.process_nodes(props.to_vec(), macro_root.clone())?,
                }
            }
            Statement::Array(values) => {
                node.statement = Statement::Array(
                    values
                        .iter()
                        .map(|x| self.process_node(x.clone(), macro_root.clone()))
                        .collect::<ResultNodeVec>()?,
                );
            }
            Statement::Bool(_) => {}
            Statement::Integer(_) => {}
            Statement::Str(_) => {}
            Statement::FILE => {
                node.statement =
                    Statement::Processed(Box::new(Statement::InternalStr(node.file.clone())), Box::new(node.statement));
            }
            Statement::LINE => {
                node.statement = Statement::Processed(
                    Box::new(Statement::Integer(if let Some(root) = macro_root {
                        root.start.0 as i32
                    } else {
                        node.start.0 as i32
                    })),
                    Box::new(node.statement),
                );
            }
            Statement::Ident(val) => {
                if let Some(s) = self.defines.get(val) {
                    node.statement = Statement::Defined(Box::new(s.clone()), Box::new(node_clone.clone()));
                }
            }
            Statement::IdentArray(val) => {
                if let Some(s) = self.defines.get(val) {
                    node.statement = Statement::Defined(Box::new(s.clone()), Box::new(node_clone.clone()));
                }
            }
            Statement::ClassDef(ident) => {
                node.statement = Statement::ClassDef(Box::new(self.process_node(*ident.clone(), macro_root)?));
            }
            Statement::ClassDelete(ident) => {
                node.statement = Statement::ClassDelete(Box::new(self.process_node(*ident.clone(), macro_root)?));
            }
            Statement::Config(nodes) => {
                node.statement = Statement::Config(self.process_nodes(nodes.to_vec(), macro_root.clone())?);
            }
            // Directives
            Statement::Define { ident, value } => {
                let mut warn_node = node_clone.clone();
                if let Some(old) = self.defines.remove(ident) {
                    warn_node.statement = Statement::Redefine(
                        format!("Redefining `{}`", ident),
                        Box::new(warn_node.statement.clone()),
                        Box::new(old),
                    );
                    self.report.warnings.push(warn_node.clone());
                };
                if let Some(old) = self.macros.remove(ident) {
                    warn_node.statement = Statement::Redefine(
                        format!("Redefining `{}`", ident),
                        Box::new(warn_node.statement.clone()),
                        Box::new(old.1),
                    );
                    self.report.warnings.push(warn_node.clone());
                };
                let data = self.process_node(*value.clone(), macro_root.clone())?;
                self.defines.insert(ident.to_string(), data);
                if ident.to_uppercase() != *ident {
                    warn_node.statement = Statement::NonUppercaseDefine(Box::new(node.statement.clone()));
                    self.report.warnings.push(warn_node.clone());
                }
            }
            Statement::DefineMacro { ident, args, value, .. } => {
                let mut warn_node = node_clone.clone();
                if let Some(old) = self.defines.remove(ident) {
                    warn_node.statement = Statement::Redefine(
                        format!("Redefining `{}`", ident),
                        Box::new(warn_node.statement.clone()),
                        Box::new(old),
                    );
                    self.report.warnings.push(warn_node.clone());
                };
                if let Some(old) = self.macros.remove(ident) {
                    warn_node.statement = Statement::Redefine(
                        format!("Redefining `{}`", ident),
                        Box::new(warn_node.statement.clone()),
                        Box::new(old.1),
                    );
                    self.report.warnings.push(warn_node.clone());
                };
                self.macros.insert(ident.to_string(), (args.to_vec(), *value.clone()));
                if ident.to_uppercase() != *ident {
                    warn_node.statement = Statement::NonUppercaseDefine(Box::new(node.statement.clone()));
                    self.report.warnings.push(warn_node.clone());
                }
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
                            let macro_body = self.process_node(
                                val.clone(),
                                if node_clone.file.starts_with("MACRO:") {
                                    macro_root.clone()
                                } else {
                                    Some(node_clone.clone())
                                },
                            )?;
                            self.defines.insert(mac_args.get(i).unwrap().to_string(), macro_body);
                        }
                        node.statement = self
                            .process_node(
                                mac_node,
                                if node_clone.file.starts_with("MACRO:") {
                                    macro_root.clone()
                                } else {
                                    Some(node_clone.clone())
                                },
                            )?
                            .statement;
                        self.defines = old_defines;
                    }
                } else {
                    node.statement = Statement::Undefined(
                        format!("Call to undefined macro `{}`", ident),
                        Box::new(node.statement.clone()),
                    );
                    self.report.errors.push(node.clone());
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
                            let val = self.process_node(
                                inner_arg.clone(),
                                if node_clone.file.starts_with("MACRO:") {
                                    macro_root.clone()
                                } else {
                                    Some(node_clone.clone())
                                },
                            )?;
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
                        Statement::Defined(Box::new(val.clone()), Box::new(node_clone.clone()))
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
                        Statement::MacroCall { .. } => {
                            match self
                                .process_node(
                                    child.clone(),
                                    if node_clone.file.starts_with("MACRO:") {
                                        macro_root.clone()
                                    } else {
                                        Some(node_clone.clone())
                                    },
                                )?
                                .statement
                            {
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
                        Statement::Defined(Box::new(val.clone()), Box::new(node_clone.clone()))
                    } else {
                        self.report.warnings.push(node_clone.clone());
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
                    self.report.warnings.push(node.clone());
                }
            }
            Statement::IfDef {
                ident,
                positive,
                negative,
            } => {
                node.statement = if self.defines.contains_key(ident) || self.macros.contains_key(ident) {
                    Statement::Inserted(self.process_nodes(positive.to_vec(), macro_root.clone())?)
                } else if let Some(n) = negative {
                    Statement::Inserted(self.process_nodes(n.to_vec(), macro_root.clone())?)
                } else {
                    Statement::Gone
                };
            }
            // Ignored
            Statement::Char(_) => {}
            Statement::Float(_) => {}
            Statement::Gone => {}
            Statement::Inserted(_) => {}
            Statement::InternalStr(_) => {}
            Statement::Defined(_, _) => {}
            Statement::InvalidCall(_, _) => {}
            Statement::Processed(_, _) => {}
            Statement::Undefined(_, _) => {}
            // Warnings & erors
            Statement::NonUppercaseDefine(_) => {}
            Statement::Redefine(_, _, _) => {}
        }
        Ok(node)
    }

    pub fn tokens(&self, text: String) -> Result<String, ArmaLintError> {
        let s = text.clone();
        let mut output = Vec::new();
        for token in s.trim().split(' ') {
            if token.starts_with('#') {
                let ident = remove_first(token).unwrap();
                let data = if let Some(v) = self.defines.get(ident) {
                    super::get_ident(v.statement.clone())?
                } else {
                    ident.to_string()
                };
                output.push(data.to_string());
            } else if token.contains("##") {
                let token_parts = token.split("##");
                let mut part_str = Vec::new();
                for part in token_parts {
                    part_str.push(if let Some(v) = self.defines.get(part) {
                        super::get_ident(v.statement.clone())?
                    } else {
                        part.to_string()
                    });
                }
                output.push(part_str.join(""));
            } else {
                output.push(if let Some(v) = self.defines.get(token) {
                    super::get_ident(v.statement.clone())?
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
