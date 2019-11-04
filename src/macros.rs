#[macro_export]
macro_rules! repeat {
    ($s:expr, $n:expr) => {{
        std::iter::repeat($s).take($n).collect::<String>()
    }};
}
