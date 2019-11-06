macro_rules! open_file {
    ($e:expr) => {
        std::fs::File::open(&$e).map_err(|source| {
            crate::ArmaLintError::PATH(crate::IOPathError {
                path: std::path::PathBuf::from(&$e),
                source,
            })
        })
    };
}

macro_rules! create_file {
    ($e:expr) => {
        std::fs::File::create(&$e).map_err(|source| {
            crate::ArmaLintError::PATH(crate::IOPathError {
                path: std::path::PathBuf::from(&$e),
                source,
            })
        })
    };
}
