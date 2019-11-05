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
    ) -> Result<(Node, Vec<(String, Option<(String, usize)>, String)>), ArmaLintError>
    where
        F: Fn(&str) -> Result<String, std::io::Error> + Copy,
    {
        let mut included: Vec<(String, Option<(String, usize)>, String)> = Vec::new();
        let node = Node {
            file: file.to_string(),
            start: (pair.as_span().start_pos().pos(), pair.as_span().start_pos().line_col()),
            end: (pair.as_span().end_pos().pos(), pair.as_span().end_pos().line_col()),
            line: pair.as_span().as_str().to_string(),
            statement: match pair.as_rule() {
                Rule::config => Statement::Config(
                    pair.into_inner()
                        .map(|x| {
                            let r = Node::from_expr(file, source, x, resolver);
                            if let Ok((n, i)) = r {
                                i.iter().for_each(|x| included.push(x.clone()));
                                Ok(n)
                            } else {
                                Err(r.err().unwrap())
                            }
                        })
                        .collect::<ResultNodeVec>()?,
                ),
                Rule::class => {
                    let mut parts = pair.into_inner();
                    Statement::Class {
                        ident: Box::new({
                            let (n, i) = Node::from_expr(file, source, parts.next().unwrap(), resolver)?;
                            i.iter().for_each(|x| included.push(x.clone()));
                            n
                        }),
                        extends: None,
                        props: parts
                            .map(|x| {
                                let r = Node::from_expr(file, source, x, resolver);
                                if let Ok((n, i)) = r {
                                    i.iter().for_each(|x| included.push(x.clone()));
                                    Ok(n)
                                } else {
                                    Err(r.err().unwrap())
                                }
                            })
                            .collect::<ResultNodeVec>()?,
                    }
                }
                Rule::classextends => {
                    let mut parts = pair.into_inner();
                    Statement::Class {
                        ident: Box::new({
                            let (n, i) = Node::from_expr(file, source, parts.next().unwrap(), resolver)?;
                            i.iter().for_each(|x| included.push(x.clone()));
                            n
                        }),
                        extends: Some(Box::new({
                            let (n, i) = Node::from_expr(file, source, parts.next().unwrap(), resolver)?;
                            i.iter().for_each(|x| included.push(x.clone()));
                            n
                        })),
                        props: parts
                            .map(|x| {
                            let r = Node::from_expr(file, source, x, resolver);
                                if let Ok((n, i)) = r {
                                    i.iter().for_each(|x| included.push(x.clone()));
                                    Ok(n)
                                } else {
                                    Err(r.err().unwrap())
                                }
                            })
                            .collect::<ResultNodeVec>()?,
                    }
                }
                Rule::classdef => Statement::ClassDef(Box::new({
                    let (n, i) = Node::from_expr(file, source, pair.into_inner().next().unwrap(), resolver)?;
                    i.iter().for_each(|x| included.push(x.clone()));
                    n
                })),
                Rule::classdelete => Statement::ClassDelete(Box::new({
                    let (n, i) = Node::from_expr(file, source, pair.into_inner().next().unwrap(), resolver)?;
                    i.iter().for_each(|x| included.push(x.clone()));
                    n
                })),
                Rule::prop => {
                    let mut parts = pair.into_inner();
                    Statement::Property {
                        ident: Box::new({
                            let (n, i) = Node::from_expr(file, source, parts.next().unwrap(), resolver)?;
                            i.iter().for_each(|x| included.push(x.clone()));
                            n
                        }),
                        value: Box::new({
                            let (n, i) = Node::from_expr(file, source, parts.next().unwrap(), resolver)?;
                            i.iter().for_each(|x| included.push(x.clone()));
                            n
                        }),
                        expand: false,
                    }
                }
                Rule::propexpand => {
                    let mut parts = pair.into_inner();
                    Statement::Property {
                        ident: Box::new({
                            let (n, i) = Node::from_expr(file, source, parts.next().unwrap(), resolver)?;
                            i.iter().for_each(|x| included.push(x.clone()));
                            n
                        }),
                        value: Box::new({
                            let (n, i) = Node::from_expr(file, source, parts.next().unwrap(), resolver)?;
                            i.iter().for_each(|x| included.push(x.clone()));
                            n
                        }),
                        expand: true,
                    }
                }
                Rule::bool => Statement::Bool(pair.as_str() == "true"),
                Rule::array => Statement::Array(
                    pair.into_inner()
                        .map(|x| {
                            let r = Node::from_expr(file, source, x, resolver);
                            if let Ok((n, i)) = r {
                                i.iter().for_each(|x| included.push(x.clone()));
                                Ok(n)
                            } else {
                                Err(r.err().unwrap())
                            }
                        })
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
                        .map(|x| {
                            let r = Node::from_expr(file, source, x, resolver);
                            if let Ok((n, i)) = r {
                                i.iter().for_each(|x| included.push(x.clone()));
                                Ok(n)
                            } else {
                                Err(r.err().unwrap())
                            }
                        })
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
                    let content = &resolver(filename)?;
                    included.push((filename.to_string(), None, content.to_string()));
                    super::parse_with_resolver(filename, content, resolver)
                        .unwrap()
                        .config
                        .statement
                }
                Rule::define => {
                    let mut parts = pair.into_inner();
                    Statement::Define {
                        ident: String::from(parts.next().unwrap().as_str()),
                        value: Box::new({
                            let (n, i) = Node::from_expr(file, source, parts.next().unwrap(), resolver)?;
                            i.iter().for_each(|x| included.push(x.clone()));
                            n
                        }),
                    }
                }
                Rule::define_macro => {
                    let mut parts = pair.into_inner();
                    let ident = parts.next().unwrap().as_str();
                    let args = parts.next().unwrap();
                    let body = parts.next().unwrap();
                    Statement::DefineMacro {
                        ident: ident.to_string(),
                        args: args.into_inner()
                            .map(|x| String::from(x.as_str()))
                            .collect::<Vec<String>>(),
                        value: {
                            if let Ok(stmt) = super::parse_with_resolver(
                                &format!("MACRO:{}", ident),
                                &format!("{};", body.as_str().trim_end_matches('\n').replace("\\\n", "\n")),
                                resolver,
                            ) {
                                included.push((format!("MACRO:{}", ident), Some((file.to_string(), body.as_span().start_pos().line_col().0)), body.as_str().trim_end_matches('\n').replace("\\\n", "\n")));
                                Box::new(stmt.config)
                            } else {
                                Box::new({
                                    let (n, i) = Node::from_expr(file, source, body.clone(), resolver)?;
                                    i.iter().for_each(|x| included.push(x.clone()));
                                    n
                                })
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
                            .map(|x| {
                                let r = Node::from_expr(file, source, x, resolver);
                                if let Ok((n, i)) = r {
                                    i.iter().for_each(|x| included.push(x.clone()));
                                    Ok(n)
                                } else {
                                    Err(r.err().unwrap())
                                }
                            })
                            .collect::<ResultNodeVec>()?,
                    }
                }
                Rule::macro_call_arg => Statement::MacroCallArg(
                    pair.into_inner()
                        .map(|x| {
                            let r = Node::from_expr(file, source, x, resolver);
                            if let Ok((n, i)) = r {
                                i.iter().for_each(|x| included.push(x.clone()));
                                Ok(n)
                            } else {
                                Err(r.err().unwrap())
                            }
                        })
                        .collect::<ResultNodeVec>()?,
                ),
                Rule::macro_arg_char => Statement::Char(pair.as_str().chars().nth(0).unwrap()),
                Rule::define_macro_body => Statement::MacroBody(pair.as_str().trim_end_matches('\n').to_owned()),
                Rule::undef => Statement::Undefine(pair.into_inner().next().unwrap().as_str().to_string()),
                Rule::ifdef => {
                    let mut parts = pair.into_inner();
                    Statement::IfDef {
                        ident: String::from(parts.next().unwrap().as_str()),
                        positive: parts
                            .next()
                            .unwrap()
                            .into_inner()
                            .map(|x| {
                                let r = Node::from_expr(file, source, x, resolver);
                                if let Ok((n, i)) = r {
                                    i.iter().for_each(|x| included.push(x.clone()));
                                    Ok(n)
                                } else {
                                    Err(r.err().unwrap())
                                }
                            })
                            .collect::<ResultNodeVec>()?,
                        negative: {
                            if let Some(part) = parts.next() {
                                Some(
                                    part.into_inner()
                                        .map(|x| {
                                            let r = Node::from_expr(file, source, x, resolver);
                                            if let Ok((n, i)) = r {
                                                i.iter().for_each(|x| included.push(x.clone()));
                                                Ok(n)
                                            } else {
                                                Err(r.err().unwrap())
                                            }
                                        })
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
        };
        Ok((node, included))
    }
}
