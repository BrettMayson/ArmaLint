use armalint::execute;

fn main() {
    let args: Vec<_> = std::env::args().collect();
    execute(&args).unwrap();
}
