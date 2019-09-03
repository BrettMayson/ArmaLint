use super::{DyadicVerb, Rule};

#[derive(Debug, Clone)]
pub enum Node {
    Statement(Box<Node>),
    Integer(i32),
    Str(String),
    Ident(String),
    Variable { ident: String, expr: Box<Node>, private: bool },
    Nular(String),
    Unary,
    Binary,
    Expression(Box<Node>),
    Terms(Vec<Node>),
    Code(Vec<Node>),
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
            Rule::stmt => Node::Statement(Box::new(Node::from_expr(pair.into_inner().next().unwrap()))),
            Rule::expr => Node::from_expr(pair.into_inner().next().unwrap()),
            Rule::terms => {
                Node::Terms(pair.into_inner().map(Node::from_expr).collect())
            },
            Rule::code => {
                let mut stmts = Vec::new();
                for n in pair.into_inner() {
                    stmts.push(Node::from_expr(n))
                }
                Node::Code(stmts)
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
                println!("Unimplement Expr: {:#?}", pair);
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
            Node::Statement(node) => {
                node.replace_ident(ident, value);
            }
            Node::Ident(s) => {
                if s == ident {
                    *s = value.to_string();
                }
            }
            _ => {}
        }
    }
}

impl std::fmt::Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Node::Terms(terms) => {
                let mut output = String::new();
                for (i, term) in terms.iter().enumerate() {
                    if i != 0 { output.push_str(" ") }
                    output.push_str(&term.to_string());
                }
                write!(f, "{}", output)
            },
            Node::Variable {ident, expr, private} => {
                if *private {
                    write!(f, "private {} = {}", ident, expr)
                } else {
                    write!(f, "{} = {}", ident, expr)
                }
            },
            Node::If {expr, positive, negative} => {
                write!(f, "if ({}) then {}", expr, positive)?;
                if let Some(neg) = negative {
                    write!(f, " else {}", neg)?;
                }
                Ok(())
            },
            Node::Statement(v) => writeln!(f, "{};", v),
            Node::While {expr, stmt} => write!(f, "while {} do {}", expr, stmt),
            Node::DyadicOp {verb, lhs, rhs} => write!(f, "{} {} {}", lhs, verb, rhs),
            Node::Code(terms) => write!(f, "{{\n{}}}", indent!(terms.iter().map(|t| t.to_string()).collect::<Vec<String>>().join("\n"))),
            Node::Ident(s) => write!(f, "{}", s.to_string()),
            Node::Integer(v) => write!(f, "{}", v),
            Node::Str(v) => write!(f, "{}", v),
            _ => { 
                println!("{:?}", self);
                unimplemented!()
            }
        }
    }
}
