pub trait PrintableError<T, E> {
    fn unwrap_or_print(self) -> T;
}
impl<T, E: std::fmt::Debug + std::fmt::Display> PrintableError<T, E> for Result<T, E> {
    fn unwrap_or_print(self) -> T {
        if let Err(error) = &self {
            println!("{}", error);
            std::process::exit(1);
        }
        self.unwrap()
    }
}

#[derive(Debug)]
pub enum ArmaLintError {
    ParsingError { positives: Vec<String>, negatives: Vec<String>, position: pest::error::LineColLocation },
    InvalidInput(String),
    InvalidProperty(String),
    NotProcessed,
    NotRoot,

    // Wrappers
    IO(std::io::Error),
    PATH(IOPathError),
}

impl std::fmt::Display for ArmaLintError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            ArmaLintError::IO(ref err) => write!(f, "IO error: {}", err),
            ArmaLintError::PATH(ref err) => write!(f, "IO error {}: {}", err.path.display(), err.source),
            ArmaLintError::NotProcessed => write!(f, "Attempt to perform action on non-processed AST"),
            ArmaLintError::NotRoot => write!(f, "The root of the AST is required"),
            ArmaLintError::InvalidInput(ref err) => write!(f, "Invalid Input: {}", err),
            ArmaLintError::InvalidProperty(ref err) => write!(f, "Invalid Property: {}", err),
            ArmaLintError::ParsingError {ref positives, ref position, .. } => write!(f, "Expected {:?} at {:?}", positives, position)
        }
    }
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
