use std::io::Read;
use std::path::PathBuf;

use crate::{ArmaLintError, Command};

pub struct Lint {}
impl Command for Lint {
    fn register(&self) -> clap::App {
        clap::SubCommand::with_name("lint")
            .version(*crate::VERSION)
            .about("Lint a file")
            .arg(clap::Arg::with_name("file").help("File to lint").required(true))
    }

    fn run(&self, args: &clap::ArgMatches) -> Result<(), ArmaLintError> {
        let path = PathBuf::from(args.value_of("file").unwrap());
        let mut f = open_file!(path)?;
        match path.extension().unwrap().to_str().unwrap() {
            "cpp" | "hpp" => {
                let mut content = String::new();
                f.read_to_string(&mut content)?;
                let ast = crate::config::parse(args.value_of("file").unwrap(), &content)?;
                let mut preprocessor = crate::config::PreProcessor::new();
                let processed = preprocessor.process(ast)?;
                let report = processed.report.clone().unwrap();
                println!("Syntax: Valid");
                println!("PreProcessor: {}", if processed.valid() { "Valid" } else { "Invalid" });
                for warning in report.warnings {
                    node_warning!(processed.files, warning);
                }
                for error in report.errors {
                    node_error!(processed.files, error);
                }
            }
            _ => {
                return Err(ArmaLintError::InvalidInput(format!(
                    "Unable to process `{}` files",
                    path.extension().unwrap().to_str().unwrap()
                )))
            }
        }
        Ok(())
    }
}
