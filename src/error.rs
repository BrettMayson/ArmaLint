#[derive(Debug)]
pub enum ArmaLintError {
    ParsingError {
        positives: Vec<String>,
        negatives: Vec<String>,
    },
    InvalidInput(String),
    InvalidProperty(String),
    NotProcessed,
    NotRoot,

    // Wrappers
    IO(std::io::Error),
}

impl From<std::io::Error> for ArmaLintError {
    fn from(err: std::io::Error) -> ArmaLintError {
        ArmaLintError::IO(err)
    }
}
