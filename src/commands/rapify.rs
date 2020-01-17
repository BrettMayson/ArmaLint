use std::io::Read;
use std::path::PathBuf;

use crate::{ArmaLintError, Command};

pub struct Rapify {}
impl Command for Rapify {
    fn register(&self) -> clap::App {
        clap::SubCommand::with_name("rapify")
            .version(*crate::VERSION)
            .about("Rapify a file")
            .arg(clap::Arg::with_name("file").help("File to rapify").required(true))
    }

    fn run(&self, args: &clap::ArgMatches) -> Result<(), ArmaLintError> {
        let path = PathBuf::from(args.value_of("file").unwrap());
        let mut f = open_file!(path)?;
        match path.extension().unwrap().to_str().unwrap() {
            "cpp" | "hpp" => {
                let mut content = String::new();
                f.read_to_string(&mut content)?;
                let ast = crate::config::parse(args.value_of("file").unwrap(), &content)?;
                let processed = ast.process()?;
                println!("{:?}", processed);
                // println!("Syntax: Valid");
                println!("PreProcessor: {}", if processed.valid() { "Valid" } else { "Invalid" });
                let report = processed.report.clone().unwrap();
                for warning in report.warnings {
                    node_warning!(processed.files, warning);
                }
                for error in report.errors {
                    node_error!(processed.files, error);
                }

                let simple = crate::config::rapify::Config::from_ast(processed).unwrap();
                let mut rapified = std::io::Cursor::new(Vec::new());
                simple.write_rapified(&mut rapified).unwrap();
                use std::io::Write;
                let mut out = create_file!("out.rap").unwrap();
                out.write(rapified.get_ref());
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
