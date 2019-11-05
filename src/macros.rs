#[macro_export]
macro_rules! repeat {
    ($s:expr, $n:expr) => {{
        std::iter::repeat($s).take($n).collect::<String>()
    }};
}

#[macro_export]
macro_rules! iformat {
    ($e:expr, $($p:ident),*) => {
        format!($e, $($p = $p,)*);
    };
}

// Messages

#[macro_export]
macro_rules! get_message {
    ($n:ident) => {
        match $n.statement {
            crate::config::Statement::Undefined(ref v, ref o) => {
                match &**o {
                    crate::config::Statement::Undefine(ref i) => {
                        (
                            v.clone(),
                            Some(7),
                            i.len(),
                            Some("remove this line"),
                            crate::HelpType::Help,
                        )
                    }
                    crate::config::Statement::MacroCall { ident, .. } => {
                        (
                            v.clone(),
                            Some(0),
                            ident.len(),
                            None,
                            crate::HelpType::None,
                        )
                    },
                    _ => panic!("Not an error / warning: {:#?}", $n)
                }
            },
            _ => {panic!("No way to warn for {:?}", $n)}
        }
    };
}

#[macro_export]
macro_rules! display_info {
    ($n:ident, $i:expr) => {
        let (_, help_start, help_len, help_message, help_type) = $i;
        let arrow = "-->".blue().bold();
        let sep = "|".blue().bold();
        let file = $n.file.clone();
        let line = ($n.start.1).0.to_string().blue().bold();
        let space = repeat!(" ", line.len() + 2);
        let content = $n.line.clone();
        let range = format!("{}:{}-{}:{}", ($n.start.1).0,  ($n.start.1).1, ($n.end.1).0,  ($n.end.1).1);
        println!("{}", crate::iformat!(
            "  {arrow} {file} {range}\n{space}{sep}\n {line} {sep} {content}",
            arrow, file, range, sep, line, space, content
        ));
        if let Some(start) = help_start {
            let arrows = match help_type {
                crate::HelpType::Help => repeat!("^", help_len).yellow(),
                crate::HelpType::Note => repeat!("^", help_len).blue(),
                crate::HelpType::None => repeat!("^", help_len).blue(),
            }.bold();
            if let Some(help) = help_message {
                let help = match help_type {
                    crate::HelpType::Help => format!("help: {}", help).yellow(),
                    crate::HelpType::Note => format!("note: {}", help).blue(),
                    crate::HelpType::None => String::new().bold(),
                }.bold();
                println!("{}{} {}{} {}", space, sep, repeat!(" ", start), arrows, help);
            } else {
                println!("{}{} {}{}", space, sep, repeat!(" ", start), arrows);
            }
        } else {
            println!("{}{}", space, sep);
        }
    };
}

#[macro_export]
macro_rules! node_warning {
    ($n:ident) => {
        use colored::Colorize;
        let (message, help_start, help_len, help_message, help_type) = get_message!($n);
        let message = message.bold();
        println!("\n{}: {}", "warning".yellow().bold(), message);
        display_info!($n, (message, help_start, help_len, help_message, help_type));
    };
}

#[macro_export]
macro_rules! node_error {
    ($n:ident) => {
        use colored::Colorize;
        let (message, help_start, help_len, help_message, help_type) = get_message!($n);
        let message = message.bold();
        println!("\n{}: {}", "error".red().bold(), message);
        display_info!($n, (message, help_start, help_len, help_message, help_type));
    };
}

// FS
#[macro_export]
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

#[macro_export]
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

#[macro_export]
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

#[macro_export]
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

#[macro_export]
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

#[macro_export]
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
