use armalint::execute;
use armalint::error::PrintableError;

fn main() {
    let args: Vec<_> = std::env::args().collect();
    execute(&args).unwrap_or_print();
}
