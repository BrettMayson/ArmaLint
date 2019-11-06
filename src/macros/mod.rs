#[macro_use]
mod fs;

#[macro_use]
mod node;

macro_rules! repeat {
    ($s:expr, $n:expr) => {{
        std::iter::repeat($s).take($n).collect::<String>()
    }};
}

macro_rules! iformat {
    ($e:expr, $($p:ident),*) => {
        format!($e, $($p = $p,)*);
    };
}

macro_rules! warn {
    ($e:expr) => {
        use colored::Colorize;
        println!("{} {}", "Warning:".yellow().bold(), $e);
    };
}

macro_rules! error {
    ($e:expr) => {
        use colored::Colorize;
        println!("{} {}", "Error:".red().bold(), $e);
    };
}
