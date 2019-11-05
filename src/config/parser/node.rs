use super::{Rule, Statement};
use crate::ArmaLintError;

#[derive(Debug, Clone, PartialEq)]
pub struct Node {
    pub file: String,
    pub start: (usize, (usize, usize)),
    pub end: (usize, (usize, usize)),
    pub line: String,
    pub statement: Statement,
}

type ResultNodeVec = Result<Vec<Node>, ArmaLintError>;

impl Node {
    pub fn from_expr<F>(
        file: &str,
        source: &str,
        pair: pest::iterators::Pair<Rule>,
        resolver: F,
    ) -> Result<Node, ArmaLintError>
    where
        F: Fn(&str) -> Result<String, std::io::Error> + Copy,
    {
        Ok(Node {
            file: file.to_string(),
            start: (pair.as_span().start_pos().pos(), pair.as_span().start_pos().line_col()),
            end: (pair.as_span().end_pos().pos(), pair.as_span().end_pos().line_col()),
            line: pair.as_span().as_str().to_string(),
            statement: match pair.as_rule() {
                Rule::config => Statement::Config(
                    pair.into_inner()
                        .map(|x| Node::from_expr(file, source, x, resolver))
                        .collect::<ResultNodeVec>()?,
                ),
                Rule::class => {
                    let mut parts = pair.into_inner();
                    Statement::Class {
                        ident: Box::new(Node::from_expr(file, source, parts.next().unwrap(), resolver)?),
                        extends: None,
                        props: parts
                            .map(|x| Node::from_expr(file, source, x, resolver))
                            .collect::<ResultNodeVec>()?,
                    }
                }
                Rule::classextends => {
                    let mut parts = pair.into_inner();
                    Statement::Class {
                        ident: Box::new(Node::from_expr(file, source, parts.next().unwrap(), resolver)?),
                        extends: Some(Box::new(Node::from_expr(file, source, parts.next().unwrap(), resolver)?)),
                        props: parts
                            .map(|x| Node::from_expr(file, source, x, resolver))
                            .collect::<ResultNodeVec>()?,
                    }
                }
                Rule::classdef => Statement::ClassDef(Box::new(Node::from_expr(
                    file,
                    source,
                    pair.into_inner().next().unwrap(),
                    resolver,
                )?)),
                Rule::classdelete => Statement::ClassDelete(Box::new(Node::from_expr(
                    file,
                    source,
                    pair.into_inner().next().unwrap(),
                    resolver,
                )?)),
                Rule::prop => {
                    let mut parts = pair.into_inner();
                    Statement::Property {
                        ident: Box::new(Node::from_expr(file, source, parts.next().unwrap(), resolver)?),
                        value: Box::new(Node::from_expr(file, source, parts.next().unwrap(), resolver)?),
                        expand: false,
                    }
                }
                Rule::propexpand => {
                    let mut parts = pair.into_inner();
                    Statement::Property {
                        ident: Box::new(Node::from_expr(file, source, parts.next().unwrap(), resolver)?),
                        value: Box::new(Node::from_expr(file, source, parts.next().unwrap(), resolver)?),
                        expand: true,
                    }
                }
                Rule::bool => Statement::Bool(pair.as_str() == "true"),
                Rule::array => Statement::Array(
                    pair.into_inner()
                        .map(|x| Node::from_expr(file, source, x, resolver))
                        .collect::<ResultNodeVec>()?,
                ),
                Rule::float => Statement::Float(pair.as_str().parse().unwrap()),
                Rule::integer => Statement::Integer(pair.as_str().parse().unwrap()),
                Rule::string => Statement::Str(String::from(pair.as_str())),
                Rule::ident => Statement::Ident(String::from(pair.as_str())),
                Rule::identarray => Statement::IdentArray(String::from(pair.into_inner().next().unwrap().as_str())),
                Rule::char => Statement::Char(pair.as_str().chars().nth(0).unwrap()),
                Rule::unquoted => Statement::Unquoted(
                    pair.into_inner()
                        .map(|x| Node::from_expr(file, source, x, resolver))
                        .collect::<ResultNodeVec>()?,
                ),
                // Special
                Rule::special => match pair.as_str() {
                    "__FILE__" => Statement::FILE,
                    "__LINE__" => Statement::LINE,
                    _ => panic!("Special was not handled. Please report this to ArmaLint"),
                },
                // Directives
                Rule::include => {
                    let filename = pair.into_inner().next().unwrap().as_str();
                    super::parse_with_resolver(filename, &resolver(filename)?, resolver)
                        .unwrap()
                        .config
                        .statement
                }
                Rule::define => {
                    let mut parts = pair.into_inner();
                    Statement::Define {
                        ident: String::from(parts.next().unwrap().as_str()),
                        value: Box::new(Node::from_expr(file, source, parts.next().unwrap(), resolver)?),
                    }
                }
                Rule::define_macro => {
                    let mut parts = pair.into_inner();
                    let ident = parts.next().unwrap().as_str();
                    Statement::DefineMacro {
                        ident: ident.to_string(),
                        args: parts
                            .next()
                            .unwrap()
                            .into_inner()
                            .map(|x| String::from(x.as_str()))
                            .collect::<Vec<String>>(),
                        value: {
                            let body = parts.next().unwrap();
                            if let Ok(stmt) = super::parse_with_resolver(
                                &format!("MACRO:{}", ident),
                                &format!("{};", body.as_str().replace("\\\n", "\n").trim_end()),
                                resolver,
                            ) {
                                Box::new(stmt.config)
                            } else {
                                Box::new(Node::from_expr(file, source, body, resolver)?)
                            }
                        },
                    }
                }
                Rule::macro_call => {
                    let mut parts = pair.into_inner();
                    Statement::MacroCall {
                        ident: String::from(parts.next().unwrap().as_str()),
                        args: parts
                            .next()
                            .unwrap()
                            .into_inner()
                            .map(|x| Node::from_expr(file, source, x, resolver))
                            .collect::<ResultNodeVec>()?,
                    }
                }
                Rule::macro_call_arg => Statement::MacroCallArg(
                    pair.into_inner()
                        .map(|x| Node::from_expr(file, source, x, resolver))
                        .collect::<ResultNodeVec>()?,
                ),
                Rule::macro_arg_char => Statement::Char(pair.as_str().chars().nth(0).unwrap()),
                Rule::define_macro_body => Statement::MacroBody(pair.as_str().to_owned()),
                Rule::undef => Statement::Undefine(pair.into_inner().next().unwrap().as_str().to_string()),
                Rule::ifdef => {
                    let mut parts = pair.into_inner();
                    Statement::IfDef {
                        ident: String::from(parts.next().unwrap().as_str()),
                        positive: parts
                            .next()
                            .unwrap()
                            .into_inner()
                            .map(|x| Node::from_expr(file, source, x, resolver))
                            .collect::<ResultNodeVec>()?,
                        negative: {
                            if let Some(part) = parts.next() {
                                Some(
                                    part.into_inner()
                                        .map(|x| Node::from_expr(file, source, x, resolver))
                                        .collect::<ResultNodeVec>()?,
                                )
                            } else {
                                None
                            }
                        },
                    }
                }

                // Ignored
                Rule::EOI => unimplemented!(),
                Rule::file => unimplemented!(),
                Rule::string_wrapper => unimplemented!(),
                Rule::items => unimplemented!(),
                Rule::value => unimplemented!(),
                Rule::directive => unimplemented!(),
                Rule::macro_call_args => unimplemented!(),
                Rule::define_macro_args => unimplemented!(),
                Rule::include_file => unimplemented!(),
                Rule::define_whitespace => unimplemented!(),
                Rule::conditional_block => unimplemented!(),
                Rule::COMMENT => unimplemented!(),
                Rule::WHITESPACE => unimplemented!(),
            },
        })
    }
}
