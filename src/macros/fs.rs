macro_rules! create_dir {
    ($e:expr) => {
        std::fs::create_dir_all(&$e).map_err(|source| {
            crate::ArmaLintError::PATH(crate::IOPathError {
                source,
                path: std::path::Path::new(&$e.clone()).to_path_buf(),
            })
        })
    };
}

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

macro_rules! copy_file {
    ($s:expr, $d:expr) => {
        std::fs::copy(&$s, &$d).map_err(|source| {
            crate::ArmaLintError::GENERIC(
                format!("Unable to copy: {}", source),
                format!("`{:#?}` => `{:#?}`", $s, $d),
            )
        })
    };
}

macro_rules! rename_file {
    ($s:expr, $d:expr) => {
        std::fs::rename(&$s, &$d).map_err(|source| {
            crate::ArmaLintError::GENERIC(
                format!("Unable to rename: {}", source),
                format!("`{:#?}` => `{:#?}`", $s, $d),
            )
        })
    };
}

macro_rules! remove_file {
    ($s:expr) => {
        std::fs::remove_file(&$s).map_err(|source| {
            crate::ArmaLintError::PATH(crate::IOPathError {
                path: std::path::PathBuf::from(&$s),
                source,
            })
        })
    };
}
