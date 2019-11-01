use armalint;

/*fn main() {
    let content = std::fs::read_to_string("sample.sqf").unwrap();
    let (clean, macros) = armalint::sqf::preprocess::macros(&content).unwrap();
    let ast = armalint::sqf::ast::parse(&clean).unwrap();
    let ast = armalint::sqf::preprocess::sqf(ast, macros).unwrap();
    println!("{:#?}", &ast);
    println!("{}", ast.to_string());
}*/

fn main() {
    let content = std::fs::read_to_string("sample.cpp").unwrap();
    let ast = armalint::config::ast::parse("sample.cpp", &content, |filename| {
        println!("Including {}", filename);
        std::fs::read_to_string(filename).unwrap()
    })
    .unwrap();
    let mut preprocessor = armalint::config::ast::PreProcessor::new();
    let new_ast = preprocessor.process(ast).unwrap();
    // println!("===========");
    // println!("{:#?}", new_ast);
    // println!("===========");
    preprocessor.report();
    let out = armalint::config::ast::Renderer::render(new_ast).unwrap();
    println!("==========");
    print!("{}", out);
    println!("==========");
    //println!("{:#?}", &ast);
}
