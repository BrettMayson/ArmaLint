use pest::Parser;
use pest::error::Error;

mod dyadic;
pub use dyadic::DyadicVerb;

mod node;
pub use node::Node;

#[derive(Parser)]
#[grammar = "grammars/arma3.pest"]
pub struct Arma3Parser;

#[derive(Debug, Clone)]
pub struct AST {
    ast: Vec<Node>,
}
impl AST {
    pub fn replace_ident(&mut self, ident: &str, value: &str) {
        println!("== REPLACE {}", ident);
        for node in &mut self.ast {
            println!("{:?}", node);
            node.replace_ident(ident, value);
        }
        println!("-- REPLACE");
    }

    pub fn render(&self) -> String {
        let mut output = String::new();
        for node in &self.ast {
            output.push_str(&node.to_string());
        }
        output
    }
}

impl ToString for AST {
    fn to_string(&self) -> String {
        let mut output = String::new();
        for node in &self.ast {
            output.push_str(&node.to_string());
        }
        output
    }
}

pub fn parse(source: &str) -> Result<AST, Error<Rule>> {
    let mut ast = vec![];
    
    let pairs = Arma3Parser::parse(Rule::program, source)?;
    for pair in pairs {
        match pair.as_rule() {
            Rule::stmt => ast.push(Node::from_expr(pair)),
            _ => {
                println!("Unimplement Pair: {:?}", pair);
            }
        }
    }
    Ok(AST{ast})
}
