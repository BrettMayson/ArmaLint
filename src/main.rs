use armalint;

fn main() {
    let content = std::fs::read_to_string("sample.sqf").unwrap();
    let content = armalint::preprocess::parse(&content, armalint::Context::SQF).unwrap();
    let ast = armalint::ast::parse(&content).unwrap();
    println!("{:#?}", &ast);
    println!("{}", content);
}
