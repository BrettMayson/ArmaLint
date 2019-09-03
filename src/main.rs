use armalint;

fn main() {
    let content = std::fs::read_to_string("sample.sqf").unwrap();
    let (clean, macros) = armalint::preprocess::macros(&content).unwrap();
    let ast = armalint::ast::parse(&clean).unwrap();
    let ast = armalint::preprocess::sqf(ast, macros).unwrap();
    println!("{:#?}", &ast);
    println!("{}", ast.to_string());
}
