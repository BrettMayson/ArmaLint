use std::fs::File;
use std::path::Path;

extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;
use pest::error::Error;

#[derive(Parser)]
#[grammar = "grammars/cmds.pest"]
struct CMDSParser;

#[derive(Debug)]
struct CMD {
    name: String,
    variants: Vec<Signature>,
}

#[derive(Debug)]
enum Signature {
    Nular,
    Unary(Type),
    Binary(Type, Type),
}

#[derive(Debug)]
enum Type {
    _Array(Vec<Type>),
    _Optional(String),
    _Multiple(String),
    _Single(String),
}

fn main() {
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("cmds.rs");
    let mut f = File::create(&dest_path).unwrap();

    let source = std::fs::read_to_string("sqf/chat.alcmds").unwrap();

    let mut cmds: Vec<CMD> = Vec::new();

    //let mut ast = vec![];
    let pairs = CMDSParser::parse(Rule::file, &source).unwrap();
    for pair in pairs {
        match pair.as_rule() {
            Rule::cmd => {
                println!("{:?}", pair);
                cmds.push(from_cmd(pair));
            },
            Rule::EOI => {
                println!("Done!");
            }
            _ => {
                println!("=======unimplemented=======");
                println!("{:?}", pair);
                unimplemented!()}
        }
    }
}

fn from_cmd(pair: pest::iterators::Pair<Rule>) -> CMD {
    match pair.as_rule() {
        Rule::cmd => {
            let mut inner = pair.into_inner();
            let mut cmd = CMD {
                name: inner.next().unwrap().as_str().to_owned(),
                variants: Vec::new(),
            };
            while let Some(variant) = inner.next() {
                let mut inner = variant.into_inner();
                cmd.variants.push(match inner.next().unwrap().as_str() {
                    "nular" => Signature::Nular,
                    "unary" => Signature::Unary(atype(inner.next().unwrap())),
                    "binary" => Signature::Binary(
                        atype(inner.next().unwrap()),
                        atype(inner.next().unwrap()),
                    ),
                    _ => unimplemented!()
                });
            }
            println!("CMD Added - {:#?}", cmd);
            cmd
        },
        _ => {
            println!("Unimplement Pair: {:#?}", pair);
            unimplemented!()
        }
    }
}

fn atype(pair: pest::iterators::Pair<Rule>) -> Type {
    let inner = pair.into_inner().next().unwrap();
    match inner.as_rule() {
        Rule::single => Type::_Single(inner.as_str().to_owned()),
        Rule::multiple => Type::_Multiple(inner.into_inner().next().unwrap().as_str().to_owned()),
        Rule::optional => Type::_Optional(inner.into_inner().next().unwrap().as_str().to_owned()),
        Rule::array => {
            let mut items = inner.into_inner();
            let mut types = Vec::new();
            while let Some(item) = items.next() {
                println!("Array - {:?}", item);
                types.push(atype(item));
            }
            Type::_Array(types)
        },
        _ => unimplemented!()
    }
}
