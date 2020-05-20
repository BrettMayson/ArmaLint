use super::Node;

#[derive(Debug, Clone, PartialEq, AsStaticStr)]
pub enum Statement {
    Config(Vec<Node>),
    Array(Vec<Node>),
    Float(f32),
    Integer(i32),
    Str(String),
    Bool(bool),
    Char(char),
    Unquoted(Vec<Node>),
    Property {
        ident: Box<Node>,
        value: Box<Node>,
        expand: bool,
    },
    Class {
        ident: Box<Node>,
        extends: Option<Box<Node>>,
        props: Vec<Node>,
    },
    ClassDef(Box<Node>),
    ClassDelete(Box<Node>),
    Ident(String),
    IdentArray(String),

    // Special
    FILE,
    LINE,

    // Directives
    Define {
        ident: String,
        value: Option<Box<Node>>,
    },
    DefineMacro {
        ident: String,
        args: Vec<String>,
        value: Box<Node>,
    },
    MacroCall {
        ident: String,
        args: Vec<Node>,
    },
    MacroBody(String),
    MacroCallArg(Vec<Node>),
    Undefine(String),
    IfDef {
        ident: String,
        positive: Vec<Node>,
        negative: Option<Vec<Node>>,
    },
    IfNDef {
        ident: String,
        positive: Vec<Node>,
        negative: Option<Vec<Node>>,
    },

    // Internal
    Processed(Box<Statement>, Box<Statement>),
    InternalStr(String),
    Quoted(Box<Statement>),
    Spaced(Vec<Node>),
    Bracket(Box<Node>),
    Square(Box<Node>),
    Inserted(Vec<Node>),
    // Errors
    Undefined(String, Box<Statement>),
    // Message, Original, Definition
    FlagAsIdent(String, Box<Statement>, Box<Node>),
    // Definition, Original
    Defined(Box<Node>, Box<Node>),
    // Message, Original, Definition
    InvalidCall(String, Box<Statement>, Box<Node>),
    Gone,

    // Warnings & Errors
    NonUppercaseDefine(Box<Statement>),
    Redefine(String, Box<Statement>, Option<Box<Node>>),
}

impl Statement {
    pub fn safe_unquoted(&self) -> bool {
        match self {
            Statement::InternalStr(_) => false,
            _ => true,
        }
    }

    pub fn string(&self) -> Option<String> {
        let mut ret = String::new();
        Some(
            match self {
                Statement::InternalStr(ref val) => val,
                Statement::Str(ref val) => val,
                Statement::Inserted(ref nodes) => {
                    for n in nodes {
                        ret.push_str(&n.statement.string().unwrap());
                    }
                    &ret
                }
                Statement::Unquoted(ref nodes) => {
                    for n in nodes {
                        ret.push_str(&n.statement.string().unwrap());
                    }
                    &ret
                }
                Statement::Spaced(ref nodes) => {
                    let mut retn = Vec::new();
                    for n in nodes {
                        retn.push(n.statement.string().unwrap());
                    }
                    ret = retn.join(" ");
                    &ret
                }
                _ => {
                    println!("shouldn't be asking for string of {:?}", self);
                    return None;
                }
            }
            .clone(),
        )
    }
}
