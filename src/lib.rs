extern crate pest;
#[macro_use]
extern crate pest_derive;

extern crate strum;
#[macro_use]
extern crate strum_macros;

#[macro_use]
pub mod macros;

mod error;
pub use error::ArmaLintError;

pub mod io;

//pub mod sqf;
pub mod config;
