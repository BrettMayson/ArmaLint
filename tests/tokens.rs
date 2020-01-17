use armalint;

const FILE: &str = "tests/tokens.cpp";
const FILENAME: &str = "tokens.cpp";

#[test]
fn simplify() {
    let content = std::fs::read_to_string(FILE).unwrap();
    let ast = armalint::config::parse(FILENAME, &content).unwrap();
    let mut preprocessor = armalint::config::PreProcessor::new();
    let processed = preprocessor.process(ast).unwrap();
    armalint::config::rapify::Config::from_ast(processed).unwrap();
}

#[test]
fn rapify() {
    let content = std::fs::read_to_string(FILE).unwrap();
    let ast = armalint::config::parse(FILENAME, &content).unwrap();
    let mut preprocessor = armalint::config::PreProcessor::new();
    let processed = preprocessor.process(ast).unwrap();
    let simple = armalint::config::rapify::Config::from_ast(processed).unwrap();
    let mut rapified = std::io::Cursor::new(Vec::new());
    simple.write_rapified(&mut rapified).unwrap();
    use std::io::Read;
    let mut test_against = Vec::new();
    std::fs::File::open("tests/tokens.bin")
        .unwrap()
        .read_to_end(&mut test_against)
        .unwrap();
    assert_eq!(rapified.get_ref(), &test_against);
}
