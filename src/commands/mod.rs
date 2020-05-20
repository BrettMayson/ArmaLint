use crate::ArmaLintError;

mod lint;
pub use lint::Lint;

mod rapify;
pub use rapify::Rapify;

pub trait Command {
    // (name, description)
    fn register(&self) -> clap::App;
    fn run(&self, _args: &clap::ArgMatches) -> Result<(), ArmaLintError> {
        unimplemented!();
    }
}
