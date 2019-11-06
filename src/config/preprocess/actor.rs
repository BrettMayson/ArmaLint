use std::collections::HashMap;

use regex::Regex;

use super::super::{Node, Report, Statement};
use crate::ArmaLintError;

type ResultNodeVec = Result<Vec<Node>, ArmaLintError>;

#[derive(Default)]
pub struct Actor {
    // IDENT, (DEFINITION, VALUE?)
    pub defines: HashMap<String, (Box<Node>, Option<Box<Node>>)>,
    pub macros: HashMap<String, (Vec<String>, Node)>,
    pub report: Report,
}
impl Actor {
    pub fn new() -> Self {
        Self {
            defines: HashMap::new(),
            macros: HashMap::new(),
            report: Report::new(),
        }
    }

    pub fn process_nodes(&mut self, nodes: Vec<Node>, root_node: Option<Node>) -> ResultNodeVec {
        Ok(nodes
            .into_iter()
            .map(|x| self.process_node(x, root_node.clone()))
            .collect::<Result<Vec<Node>, ArmaLintError>>()?)
    }

    pub fn process_node(&mut self, node: Node, macro_root: Option<Node>) -> Result<Node, ArmaLintError> {
        let mut new_node = node.clone();
        match node.statement {
            Statement::Unquoted(ref nodes) => {
                let mut last = 1;
                let mut new_nodes = Vec::new();
                for n in nodes {
                    let mut insert = node.clone();
                    insert.statement = self.solve(&insert.line[last-1..((n.start.1).1 - (node.start.1).1)], &node)?;
                    new_nodes.push(insert);
                    last = (n.end.1).1;
                    new_nodes.push(self.process_node(n.clone(), Some(node.clone()))?);
                }
                println!("{} : {}", last, node.line.len());
                if last-(node.start.1).1 != node.line.len() {
                    let mut insert = node.clone();
                    println!("Fill end: {:?} - {}", insert, last);
                    insert.statement = self.solve(&insert.line[last-1-(node.start.1).1..insert.line.len()], &node)?;
                    new_nodes.push(insert);
                }
                println!("unquoted: {:?}", new_nodes);
                new_node.statement = Statement::Unquoted(new_nodes);
            }
            Statement::Property {
                ref ident,
                ref value,
                ref expand,
            } => {
                new_node.statement = Statement::Property {
                    ident: Box::new(self.process_node(*ident.clone(), macro_root.clone())?),
                    value: Box::new(self.process_node(*value.clone(), macro_root.clone())?),
                    expand: *expand,
                };
            }
            Statement::Ident(ref val) => {
                self.solve(val, &node)?;
            }
            // Directives
            Statement::Define { ref ident, ref value } => {
                self.redefine(&ident, &node);
                self.defines.insert(ident.clone(), (Box::new(node.clone()), value.clone()));
            }
            Statement::DefineMacro {
                ref ident,
                ref args,
                ref value,
            } => {
                self.redefine(&ident, &node);
                self.macros.insert(ident.clone(), (args.clone(), *value.clone()));
            }
            Statement::MacroCall { ref ident, ref args } => {
                if let Some(def) = self.macros.get(ident) {
                    let (mac_args, mac_node) = def.clone();
                    if mac_args.len() != args.len() {
                        new_node.statement = Statement::InvalidCall(
                            format!(
                                "Calling macro `{}` with `{}` args, requires `{}`",
                                ident,
                                args.len(),
                                mac_args.len()
                            ),
                            Box::new(node.statement.clone()),
                            Box::new(mac_node.clone()),
                        );
                        self.report.errors.push(new_node.clone());
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
                            self.defines.insert(mac_args.get(i).unwrap().to_string(), (Box::new(val.clone()), Some(Box::new(macro_body))));
                        }
                        new_node.statement = self
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
                new_node.statement = self.solve(s, &node)?;
            }
            Statement::MacroCallArg(ref nodes) => {
                let mut last = 1;
                let mut new_nodes = Vec::new();
                for n in nodes {
                    let mut insert = node.clone();
                    insert.statement = self.solve(&insert.line[last-1..((n.end.1).1 - (node.start.1).1)], &node)?;
                    new_nodes.push(insert);
                    last = (n.end.1).1;
                    new_nodes.push(self.process_node(n.clone(), Some(node.clone()))?);
                }
                if last != node.line.len() - 1 {
                    let mut insert = node.clone();
                    insert.statement = self.solve(&insert.line[last-1..insert.line.len()], &node)?;
                    new_nodes.push(insert);
                }
                new_node.statement = Statement::Unquoted(new_nodes);
            }
            // Internal
            Statement::Inserted(ref nodes) => {}
            _ => panic!("Not able to handle: {:#?}", node),
        }
        Ok(new_node)
    }

    pub fn get_ident(&mut self, ident: &str, node: &Node) -> Result<Node, crate::ArmaLintError> {
        println!("given {:?}", ident);
        if let Some(defined) = self.defines.get(ident) {
            println!("found: {:?}", defined);
            if let Some(val) = defined.1.clone() {
                println!("defined: {:?}", val);
                return Ok(self.process_node(*val, Some(node.clone()))?)
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
        println!("solving: {}", text);
        let regex = Regex::new(r"(?m)([^/\\ .]+)").unwrap();
        let mut output = Vec::new();
        let mut last = 0;
        for cap in regex.find_iter(&text) {
            let token = &text[cap.start()..cap.end()];
            println!("CAP {:?} {:?}", cap, token);
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
        println!("RENDERED: {:?}", output);
        Ok(Statement::Inserted(output))
    }

    pub fn redefine(&mut self, ident: &str, node: &Node) {
        if let Some(old) = self.defines.get(ident) {
            let mut node = node.clone();
            node.statement = Statement::Redefine(
                format!("Redefining identifier `{}`", ident),
                Box::new(node.statement.clone()),
                old.1.clone(),
            );
            self.report.warnings.push(node);
        }
        if let Some(old) = self.macros.get(ident) {
            let mut node = node.clone();
            node.statement = Statement::Redefine(
                format!("Redefining identifier `{}`", ident),
                Box::new(node.statement.clone()),
                Some(Box::new(old.1.clone())),
            );
            self.report.warnings.push(node);
        }
    }
}

fn remove_first(s: &str) -> Option<&str> {
    s.chars().next().map(|c| &s[c.len_utf8()..])
}
