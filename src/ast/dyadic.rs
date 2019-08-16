use super::{Rule, Node};

#[derive(Debug, Clone)]
pub enum DyadicVerb {
    Plus,
    Times,
    LessThan,
    LargerThan,
    Equal,
    Minus,
    Divide,
    Power,
    Residue,
    Copy,
    LargerOf,
    LargerOrEqual,
    Shape,
}

impl DyadicVerb {
    pub fn parse(pair: pest::iterators::Pair<Rule>, lhs: Node, rhs: Node) -> Node {
        Node::DyadicOp {
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
            verb: match pair.as_str() {
                "+" => DyadicVerb::Plus,
                "*" => DyadicVerb::Times,
                "-" => DyadicVerb::Minus,
                "<" => DyadicVerb::LessThan,
                "==" => DyadicVerb::Equal,
                ">" => DyadicVerb::LargerThan,
                "%" => DyadicVerb::Divide,
                "^" => DyadicVerb::Power,
                ">=" => DyadicVerb::LargerOrEqual,
                _ => panic!("Unexpected dyadic verb: {}", pair.as_str()),
            },
        }
    }
}
