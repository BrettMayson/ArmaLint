extern crate pest;
#[macro_use]
extern crate pest_derive;

pub mod sqf;
pub mod preprocess;
pub mod ast;

pub enum Context {
    SQF,
    Config,
}
