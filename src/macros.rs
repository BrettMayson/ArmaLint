macro_rules! indent {
    ($e: expr) => {{
        let mut o = String::new();
        for l in $e.to_string().split('\n') {
            if l != "" {
                o.push_str(&format!("    {}\n", l))
            }
        }
        o
    }};
}
