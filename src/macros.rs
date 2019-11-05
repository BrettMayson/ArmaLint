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
    ($l:expr, $n:ident) => {
        match $n.statement {
            crate::config::Statement::Unquoted(ref u) => {
                if $n.line.len() != 1 && u.len() == 1 {
                    continue;
                }
                if $n.line.starts_with('#') && $n.line.find(" ").is_none() {
                    continue;
                }
                (
                    "Unquoted value".to_string(),
                    ($n.start.1).0,
                    Some(($n.start.1).1),
                    $n.line.len(),
                    Some(format!("use `\"{}\"`", $n.line)),
                    crate::HelpType::Help,
                )
            }
            crate::config::Statement::Undefined(ref v, ref o) => match &**o {
                crate::config::Statement::Undefine(ref i) => (
                    v.clone(),
                    ($n.start.1).0,
                    Some(($n.start.1).1 + 7),
                    i.len(),
                    Some("remove this line".to_string()),
                    crate::HelpType::Help,
                ),
                crate::config::Statement::MacroCall { ident, .. } => (
                    v.clone(),
                    ($n.start.1).0,
                    Some(($n.start.1).1),
                    ident.len(),
                    None,
                    crate::HelpType::Help,
                ),
                _ => panic!("Not an error / warning: {:#?}", $n),
            },
            crate::config::Statement::NonUppercaseDefine(ref i) => {
                let ident = match &**i {
                    crate::config::Statement::Define { ident, .. } => ident.clone(),
                    crate::config::Statement::DefineMacro { ident, .. } => ident.clone(),
                    _ => panic!("Non upper case: {:#?}", i),
                };
                (
                    "Use of non-uppercase characters in define identifier".to_string(),
                    ($n.start.1).0,
                    Some(($n.start.1).1 + 8),
                    ident.len(),
                    Some(format!("use `{}`", ident.to_uppercase())),
                    crate::HelpType::Help,
                )
            }
            crate::config::Statement::Redefine(ref v, ref i, ref o) => {
                let ident = match &**i {
                    crate::config::Statement::Define { ident, .. } => ident.clone(),
                    crate::config::Statement::DefineMacro { ident, .. } => ident.clone(),
                    _ => panic!("Non upper case: {:#?}", i),
                };
                (
                    v.clone(),
                    ($n.start.1).0,
                    Some(($n.start.1).1 + 8),
                    ident.len(),
                    Some(format!(
                        "old value was `{}`",
                        match &o.statement {
                            crate::config::Statement::MacroBody(v) => v.clone(),
                            crate::config::Statement::Str(v) => v.clone(),
                            _ => format!("{:?}", o.statement),
                        }
                    )),
                    crate::HelpType::Note,
                )
            }
            crate::config::Statement::InvalidCall(ref v, ref c) => {
                let ident = match &**c {
                    crate::config::Statement::MacroCall { ident, .. } => ident.clone(),
                    _ => panic!("Invalid call: {:#?}", c),
                };
                (
                    v.clone(),
                    ($n.start.1).0,
                    Some(($n.start.1).1),
                    ident.len(),
                    None,
                    crate::HelpType::Help,
                )
            }
            _ => panic!("No way to warn for {:?}", $n),
        }
    };
}

#[macro_export]
macro_rules! display_info {
    ($c:expr, $n:ident, $i:expr) => {
        let (_, help_line, help_start, help_len, help_message, help_type) = $i;
        let arrow = "-->".blue().bold();
        let sep = "|".blue().bold();
        let file = $n.file.clone();

        let (file, offset) = if file.starts_with("MACRO:") {
            let (name, off) = $c.get(&file).unwrap().0.as_ref().unwrap();
            (name.clone(), *off - 1)
        } else {
            ($n.file.clone(), 0 as usize)
        };

        let range = format!(
            "{}:{}-{}:{}",
            ($n.start.1).0 + offset,
            ($n.start.1).1,
            ($n.end.1).0 + offset,
            ($n.end.1).1
        );
        println!("{}", crate::iformat!("  {arrow} {file} {range}", arrow, file, range));
        println!("    {}", sep);
        for (line, content_line) in $c.get(&file).unwrap().1.lines().collect::<Vec<_>>().iter().enumerate() {
            let line = line + 1;
            let start = ($n.start.1).0 + offset;
            let end = ($n.end.1).0 + offset;
            if (line as i32) - (start as i32) < 0 || (line as i32) - (end as i32) > 0 {
                continue;
            }
            let line_str = line.to_string().blue().bold();
            println!(
                " {}{}{} {}",
                line_str,
                repeat!(" ", 3 - line.to_string().len()),
                sep,
                content_line
            );
            if line == help_line + offset {
                if let Some(start) = help_start {
                    let arrows = match help_type {
                        crate::HelpType::Help => repeat!("^", help_len).yellow(),
                        crate::HelpType::Note => repeat!("^", help_len).blue(),
                    }
                    .bold();
                    if let Some(help) = help_message.clone() {
                        let help = match help_type {
                            crate::HelpType::Help => format!("help: {}", help).yellow(),
                            crate::HelpType::Note => format!("note: {}", help).blue(),
                        }
                        .bold();
                        println!("    {} {}{} {}", sep, repeat!(" ", start - 1), arrows, help);
                    } else {
                        println!("    {} {}{}", sep, repeat!(" ", start - 1), arrows);
                    }
                } else {
                    println!("    {}", sep);
                }
            }
        }
    };
}

#[macro_export]
macro_rules! node_warning {
    ($c:expr, $n:ident) => {
        use colored::Colorize;
        let (message, help_line, help_start, help_len, help_message, help_type) = get_message!($c, $n);
        let message = message.bold();
        println!("\n{}: {}", "warning".yellow().bold(), message);
        display_info!(
            $c,
            $n,
            (message, help_line, help_start, help_len, help_message, help_type)
        );
    };
}

#[macro_export]
macro_rules! node_error {
    ($c:expr, $n:ident) => {
        use colored::Colorize;
        let (message, help_line, help_start, help_len, help_message, help_type) = get_message!($c, $n);
        let message = message.bold();
        println!("\n{}: {}", "error".red().bold(), message);
        display_info!(
            $c,
            $n,
            (message, help_line, help_start, help_len, help_message, help_type)
        );
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
