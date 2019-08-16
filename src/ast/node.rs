use super::{DyadicVerb, Rule};

#[derive(Debug, Clone)]
pub enum Node {
    Integer(i32),
    Str(String),
    Ident(String),
    Variable { ident: String, expr: Box<Node>, private: bool },
    Nular(String),
    Unary,
    Binary,
    Expression(Box<Node>),
    Terms(Vec<Box<Node>>),
    Code(Box<Node>),
    DyadicOp {
        verb: DyadicVerb,
        lhs: Box<Node>,
        rhs: Box<Node>,
    },
    If {
        expr: Box<Node>,
        positive: Box<Node>,
        negative: Option<Box<Node>>,
    },
    While {
        expr: Box<Node>,
        stmt: Box<Node>,
    }
}

impl Node {
    pub fn from_expr(pair: pest::iterators::Pair<Rule>) -> Node {
        match pair.as_rule() {
            Rule::expr => Node::from_expr(pair.into_inner().next().unwrap()),
            Rule::terms => {
                Node::Terms(pair.into_inner().map(|p| Node::from_expr(p)).map(|i| Box::new(i)).collect())
            },
            Rule::code => {
                Node::Code(Box::new(Node::from_expr(pair.into_inner().next().unwrap())))
            },
            Rule::ident => Node::Ident(pair.as_str().to_owned()),
            Rule::assgmtExpr => {
                let mut parts = pair.into_inner();
                Node::Variable {
                    ident: parts.next().unwrap().as_str().to_owned(),
                    expr: Box::new(Node::from_expr(parts.next().unwrap())),
                    private: false,
                }
            },
            Rule::privateAssgmtExpr => {
                let mut outer = pair.into_inner();
                let mut parts = outer.next().unwrap().into_inner();
                Node::Variable {
                    ident: parts.next().unwrap().as_str().to_owned(),
                    expr: Box::new(Node::from_expr(parts.next().unwrap())),
                    private: true,
                }
            },
            Rule::dyadicExpr => {
                let mut pair = pair.into_inner();
                let lhs = pair.next().unwrap();
                let lhs = Node::from_expr(lhs);
                let verb = pair.next().unwrap();
                let rhs = pair.next().unwrap();
                let rhs = Node::from_expr(rhs);
                DyadicVerb::parse(verb, lhs, rhs)
            },
            Rule::ifstmt => {
                let mut pair = pair.into_inner();
                let expr = pair.next().unwrap();
                let positive = pair.next().unwrap();
                let negative = pair.next();
                Node::If {
                    expr: Box::new(Node::from_expr(expr)),
                    positive: Box::new(Node::from_expr(positive)),
                    negative: if let Some(neg) = negative {
                        Some(Box::new(Node::from_expr(neg)))
                    } else {
                        None
                    }
                }
            },
            Rule::whilestmt => {
                let mut pair = pair.into_inner();
                let expr = Box::new(Node::from_expr(pair.next().unwrap()));
                let stmt = Box::new(Node::from_expr(pair.next().unwrap()));
                Node::While { expr, stmt }
            }
            Rule::integer => {
                Node::Integer(pair.as_str().parse().unwrap())
            },
            Rule::string => {
                Node::Str(pair.as_str().to_owned())
            }
            _ => {
                println!("Unimplement Pair: {:#?}", pair);
                unimplemented!()
            }
        }
    }

    pub fn replace_ident(&mut self, ident: &str, value: &str) {
        match self {
            Node::Terms(terms) => {
                for term in terms {
                    term.replace_ident(ident, value);
                }
            },
            Node::Ident(s) => {
                if s == ident {
                    *s = value.to_string();
                }
            }
            _ => {}
        }
    }
}

impl ToString for Node {
    fn to_string(&self) -> String {
        match self {
            Node::Terms(terms) => {
                let mut output = String::new();
                for (i, term) in terms.iter().enumerate() {
                    if i != 0 { output.push_str(" ") }
                    output.push_str(&term.to_string());
                }
                output
            },
            Node::Ident(s) => {
                s.to_string()
            },
            _ => { 
                println!("{:?}", self);
                unimplemented!()
            }
        }
    }
}
