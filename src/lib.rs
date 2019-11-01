extern crate pest;
#[macro_use]
extern crate pest_derive;

#[macro_use]
pub mod macros;

mod error;
pub use error::ArmaLintError;

//pub mod sqf;
pub mod config;
