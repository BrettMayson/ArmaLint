#[derive(Copy, Clone)]
pub enum BracketStyle {
    /// Linux Kernel
    /// ```
    /// class MyClass {
    ///     property = "This is the Linux style";
    /// };
    /// ```
    Linux,
    /// Allman style
    /// ```
    /// class MyClass
    /// {
    ///     property = "This is the Allman style";
    /// };
    /// ```
    Allman,
}

#[derive(Copy, Clone)]
pub enum IndentationType {
    Spaces(u8),
    Tab,
    None,
}

#[derive(Copy, Clone)]
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
