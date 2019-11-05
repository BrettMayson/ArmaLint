#[derive(Debug)]
pub enum ArmaLintError {
    ParsingError { positives: Vec<String>, negatives: Vec<String> },
    InvalidInput(String),
    InvalidProperty(String),
    NotProcessed,
    NotRoot,

    // Wrappers
    IO(std::io::Error),
    PATH(IOPathError),
}

impl From<std::io::Error> for ArmaLintError {
    fn from(err: std::io::Error) -> ArmaLintError {
        ArmaLintError::IO(err)
    }
}

#[derive(Debug)]
pub struct IOPathError {
    pub source: std::io::Error,
    pub path: std::path::PathBuf,
}
