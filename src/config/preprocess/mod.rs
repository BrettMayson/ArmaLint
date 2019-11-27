use regex::Regex;
use std::collections::HashMap;

use super::{Node, Report, ResultNodeVec, Statement, AST};
use crate::ArmaLintError;

#[derive(Default, Debug)]
pub struct PreProcessor {
    pub defines: HashMap<String, (Box<Node>, Option<Box<Node>>)>,
    pub macros: HashMap<String, (Vec<String>, Node)>,
    pub report: Report,
}
impl PreProcessor {
    pub fn new() -> Self {
        Self {
            defines: HashMap::new(),
            macros: HashMap::new(),
            report: Report::new(),
        }
    }

    pub fn process(&mut self, ast: AST) -> Result<AST, ArmaLintError> {
        let mut ast = ast.clone();
        let config = match ast.config.statement {
            Statement::Config(c) => c,
            _ => return Err(ArmaLintError::NotRoot),
        };
        ast.config.statement = Statement::Config(self.process_nodes(config, None)?);
        ast.processed = true;
        ast.report = Some(self.report.clone());
        Ok(ast)
    }

    pub fn process_nodes(&mut self, nodes: Vec<Node>, root_node: Option<Node>) -> ResultNodeVec {
        Ok(nodes
            .into_iter()
            .map(|x| self.process_node(x, root_node.clone()))
            .collect::<Result<Vec<Node>, ArmaLintError>>()?)
    }

    pub fn process_node(&mut self, node: Node, macro_root: Option<Node>) -> Result<Node, ArmaLintError> {
        let mut ret_node = node.clone();
        match node.statement {
            Statement::Config(nodes) => {
                ret_node.statement = Statement::Config(self.process_nodes(nodes.to_vec(), macro_root.clone())?);
            }
            Statement::Bool(_) => {}
            Statement::Integer(_) => {}
            Statement::Float(_) => {}
            Statement::Str(_) => {}
            Statement::Ident(ref val) => {
                ret_node.statement = self.solve(val, &node)?;
            }
            Statement::IdentArray(ref val) => {
                ret_node.statement = Statement::IdentArray(self.solve(val, &node)?.string().unwrap());
            }
            Statement::Property { ident, value, expand } => {
                ret_node.statement = Statement::Property {
                    ident: Box::new(self.process_node(*ident, macro_root.clone())?),
                    value: Box::new(self.process_node(*value, macro_root.clone())?),
                    expand,
                };
            }
            Statement::Class { ident, extends, props } => {
                ret_node.statement = Statement::Class {
                    ident: Box::new(self.process_node(*ident.clone(), macro_root.clone())?),
                    extends: if let Some(e) = extends {
                        Some(Box::new(self.process_node(*e.clone(), macro_root.clone())?))
                    } else {
                        None
                    },
                    props: self.process_nodes(props.to_vec(), macro_root.clone())?,
                }
            }
            Statement::ClassDef(ident) => {
                ret_node.statement = Statement::ClassDef(Box::new(self.process_node(*ident.clone(), macro_root)?));
            }
            Statement::Array(ref values) => {
                ret_node.statement = Statement::Array(
                    values
                        .iter()
                        .map(|x| self.process_node(x.clone(), macro_root.clone()))
                        .collect::<ResultNodeVec>()?,
                );
            }
            Statement::Unquoted(ref nodes) => {
                ret_node.statement = Statement::Unquoted(
                    nodes
                        .iter()
                        .map(|n| self.process_node(n.clone(), macro_root.clone()))
                        .collect::<ResultNodeVec>()?,
                );
            }
            Statement::Spaced(ref nodes) => {
                ret_node.statement = Statement::Spaced(
                    nodes
                        .iter()
                        .map(|n| self.process_node(n.clone(), macro_root.clone()))
                        .collect::<ResultNodeVec>()?,
                );
            }
            Statement::Define { ref ident, ref value } => {
                if ident.to_uppercase() != *ident {
                    let mut warn_node = node.clone();
                    warn_node.statement = Statement::NonUppercaseDefine(Box::new(node.statement.clone()));
                    self.report.warnings.push(warn_node);
                }
                if let Some(warn) = self.reset_ident(ident, &node) {
                    self.report.warnings.push(warn);
                }
                self.defines.insert(ident.clone(), (Box::new(node.clone()), value.clone()));
            }
            Statement::DefineMacro {
                ref ident,
                ref args,
                ref value,
            } => {
                if let Some(warn) = self.reset_ident(ident, &node) {
                    self.report.warnings.push(warn);
                }
                self.macros.insert(ident.clone(), (args.clone(), *value.clone()));
            }
            Statement::Undefine(ref ident) => {
                if self.defines.remove(ident).is_none() && self.macros.remove(ident).is_none() {
                    ret_node.statement = Statement::Undefined(
                        format!("Attempt to undefine an undefined identifier `{}`", ident),
                        Box::new(node.statement.clone()),
                    );
                    self.report.warnings.push(ret_node.clone());
                }
            }
            Statement::MacroCall { ref ident, ref args } => {
                if let Some(def) = self.macros.get(ident) {
                    let (mac_args, mac_node) = def.clone();
                    if mac_args.len() != args.len() {
                        ret_node.statement = Statement::InvalidCall(
                            format!(
                                "Calling macro `{}` with `{}` args, requires `{}`",
                                ident,
                                args.len(),
                                mac_args.len()
                            ),
                            Box::new(node.statement.clone()),
                            Box::new(mac_node.clone()),
                        );
                        self.report.errors.push(ret_node.clone());
                    } else {
                        let old_defines = self.defines.clone();
                        for (i, val) in args.iter().enumerate() {
                            let macro_body = self.process_node(
                                val.clone(),
                                if node.file.starts_with("MACRO:") {
                                    macro_root.clone()
                                } else {
                                    Some(node.clone())
                                },
                            )?;
                            self.defines.insert(
                                mac_args.get(i).unwrap().to_string(),
                                (Box::new(val.clone()), Some(Box::new(macro_body))),
                            );
                        }
                        ret_node.statement = self
                            .process_node(
                                mac_node.clone(),
                                if node.file.starts_with("MACRO:") {
                                    macro_root.clone()
                                } else {
                                    Some(node.clone())
                                },
                            )?
                            .statement;
                        self.defines = old_defines;
                    }
                }
            }
            Statement::MacroBody(ref s) => {
                ret_node.statement = self.solve(s, &node)?;
            }
            Statement::MacroCallArg(ref nodes) => {
                ret_node.statement = Statement::Spaced({
                    let mut new_nodes = Vec::new();
                    for n in nodes {
                        new_nodes.push(self.process_node(n.clone(), macro_root.clone())?);
                    }
                    new_nodes
                });
            }
            Statement::IfDef {
                ref ident,
                ref positive,
                ref negative,
            } => {
                ret_node.statement = if self.defines.contains_key(ident) || self.macros.contains_key(ident) {
                    Statement::Inserted(self.process_nodes(positive.to_vec(), macro_root.clone())?)
                } else if let Some(n) = negative {
                    Statement::Inserted(self.process_nodes(n.to_vec(), macro_root.clone())?)
                } else {
                    Statement::Gone
                };
            }
            Statement::IfNDef {
                ref ident,
                ref positive,
                ref negative,
            } => {
                ret_node.statement = if !(self.defines.contains_key(ident) || self.macros.contains_key(ident)) {
                    Statement::Inserted(self.process_nodes(positive.to_vec(), macro_root.clone())?)
                } else if let Some(n) = negative {
                    Statement::Inserted(self.process_nodes(n.to_vec(), macro_root.clone())?)
                } else {
                    Statement::Gone
                };
            }
            // Internal
            Statement::Inserted(_) => {}
            Statement::Bracket(inner) => {
                ret_node.statement = Statement::Bracket(Box::new(self.process_node(*inner, macro_root)?));
            }
            Statement::Square(inner) => {
                ret_node.statement = Statement::Square(Box::new(self.process_node(*inner, macro_root)?));
            }
            _ => unimplemented!("no glue: {:?}", node.statement),
        }
        Ok(ret_node)
    }

    fn reset_ident(&mut self, ident: &str, node: &Node) -> Option<Node> {
        let mut warn_node = node.clone();
        if let Some(old) = self.defines.remove(ident) {
            warn_node.statement = Statement::Redefine(
                format!("Redefining `{}`", ident),
                Box::new(warn_node.statement.clone()),
                old.1.clone(),
            );
            return Some(warn_node);
        };
        if let Some(old) = self.macros.remove(ident) {
            warn_node.statement = Statement::Redefine(
                format!("Redefining `{}`", ident),
                Box::new(warn_node.statement.clone()),
                Some(Box::new(old.1.clone())),
            );
            return Some(warn_node);
        };
        None
    }

    pub fn get_ident(&mut self, ident: &str, node: &Node) -> Result<Node, crate::ArmaLintError> {
        if let Some(defined) = self.defines.get(ident) {
            if let Some(val) = defined.1.clone() {
                return Ok(self.process_node(*val, Some(node.clone()))?);
            } else {
                self.report.errors.push({
                    let mut node = node.clone();
                    node.statement = Statement::FlagAsIdent(
                        format!("Attempting to use flag `{}` as a defined identifier", ident),
                        Box::new(node.statement),
                        defined.0.clone(),
                    );
                    node
                });
            }
        }
        Ok({
            let mut n = node.clone();
            n.statement = Statement::InternalStr(ident.to_string());
            n
        })
    }

    pub fn solve(&mut self, text: &str, node: &Node) -> Result<Statement, ArmaLintError> {
        let regex = Regex::new(r"(?m)([^/\\ .]+)").unwrap();
        let mut output = Vec::new();
        let mut last = 0;
        for cap in regex.find_iter(&text) {
            let token = &text[cap.start()..cap.end()];
            let gap = text[last..cap.start()].to_string();
            if !gap.is_empty() {
                output.push({
                    let mut node = node.clone();
                    node.statement = Statement::InternalStr(gap);
                    node
                });
            }
            if token.starts_with('#') && !token.starts_with("##") {
                output.push({
                    let mut n = self.get_ident(remove_first(token).unwrap(), node)?;
                    n.statement = Statement::Quoted(Box::new(n.statement));
                    n
                });
            } else if token.contains("##") {
                let token_parts = token.split("##");
                for part in token_parts {
                    output.push(self.get_ident(part, node)?);
                }
            } else {
                output.push(self.get_ident(token, node)?);
            }
            last = cap.end();
        }
        let end = text[last..text.len()].to_string();
        if !end.is_empty() {
            output.push({
                let mut node = node.clone();
                node.statement = Statement::InternalStr(end);
                node
            });
        }
        Ok(Statement::Inserted(output))
    }
}

fn remove_first(s: &str) -> Option<&str> {
    s.chars().next().map(|c| &s[c.len_utf8()..])
}
