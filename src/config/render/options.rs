#[derive(Clone, Copy, Hash, PartialEq)]
pub enum BracketStyle {
    /// Linux Kernel
    /// ```hpp
    /// class MyClass {
    ///     property = "This is the Linux style";
    /// };
    /// ```
    Linux,
    /// Allman style
    /// ```hpp
    /// class MyClass
    /// {
    ///     property = "This is the Allman style";
    /// };
    /// ```
    Allman,
}

#[derive(Clone, Copy, Hash, PartialEq)]
pub enum IndentationType {
    Spaces(u8),
    Tab,
    None,
}

#[derive(Clone, Copy, Hash, PartialEq)]
pub struct RenderOptions {
    pub bracket_style: BracketStyle,
    pub indentation_type: IndentationType,
}

impl Default for RenderOptions {
    fn default() -> Self {
        Self {
            bracket_style: BracketStyle::Allman,
            indentation_type: IndentationType::Spaces(4),
        }
    }
}

#[test]
fn test_brackets() {
    let content = std::fs::read_to_string("tests/basic.cpp").unwrap();
    let ast = crate::config::parse("basic.cpp", &content).unwrap();
    let mut preprocessor = crate::config::PreProcessor::new();
    let processed = preprocessor.process(ast).unwrap();
    // Test Linux Brackets
    {
        let linux_options = RenderOptions {
            bracket_style: BracketStyle::Linux,
            indentation_type: IndentationType::Spaces(4),
        };
        let renderer = super::Renderer::new(linux_options);
        assert_eq!(
            renderer.render(processed.clone()).unwrap(),
            r##"class Test;
class TestClass: Test {
    array[] = {1, 3, 5};
    deepArray[] = {{1, 2}, {3, 4}};
    someString = "This is some string";
    someNumber = 123;
    someFloat = 3.14;
};"##
        );
    }
    // Test Allman Brackets
    {
        let linux_options = RenderOptions {
            bracket_style: BracketStyle::Allman,
            indentation_type: IndentationType::Spaces(4),
        };
        let renderer = super::Renderer::new(linux_options);
        assert_eq!(
            renderer.render(processed).unwrap(),
            r##"class Test;
class TestClass: Test
{
    array[] = {1, 3, 5};
    deepArray[] = {{1, 2}, {3, 4}};
    someString = "This is some string";
    someNumber = 123;
    someFloat = 3.14;
};"##
                .to_string()
        );
    }
}

#[test]
fn test_indents() {
    let content = std::fs::read_to_string("tests/basic.cpp").unwrap();
    let ast = crate::config::parse("basic.cpp", &content).unwrap();
    let mut preprocessor = crate::config::PreProcessor::new();
    let processed = preprocessor.process(ast).unwrap();
    // Test Tabs
    {
        let linux_options = RenderOptions {
            bracket_style: BracketStyle::Linux,
            indentation_type: IndentationType::Tab,
        };
        let renderer = super::Renderer::new(linux_options);
        assert_eq!(
            renderer.render(processed.clone()).unwrap(),
            r##"class Test;
class TestClass: Test {
    array[] = {1, 3, 5};
    deepArray[] = {{1, 2}, {3, 4}};
    someString = "This is some string";
    someNumber = 123;
    someFloat = 3.14;
};"##
                .replace("    ", "\t")
        );
    }
    // Test Spaces
    {
        let linux_options = RenderOptions {
            bracket_style: BracketStyle::Linux,
            indentation_type: IndentationType::Spaces(4),
        };
        let renderer = super::Renderer::new(linux_options);
        assert_eq!(
            renderer.render(processed).unwrap(),
            r##"class Test;
class TestClass: Test {
    array[] = {1, 3, 5};
    deepArray[] = {{1, 2}, {3, 4}};
    someString = "This is some string";
    someNumber = 123;
    someFloat = 3.14;
};"##
                .to_string()
        );
    }
}
