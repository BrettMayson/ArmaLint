use regex::Regex;

use crate::ast::AST;
use crate::Context;

#[derive(Debug, Clone)]
pub struct Macro {
    pub name: String,
    pub args: Vec<String>,
    pub expr: String,
    pub ast: AST,
}
impl Macro {
    pub fn args(&self) -> String {
        let mut output = String::new();
        for i in 0..self.args.len() {
            if i != 0 {
                output.push(',');
            }
            output.push_str("(.+?)");
        }
        output
    }
}

pub fn parse(content: &str, context: Context) -> Result<String, String> {
    // #define
    let re_define = Regex::new(r"(?m)#define(?:\s+?)([^\s]+?)\((.+)\)\s+?(.+)").unwrap();

    let mut macros = Vec::new();

    for cap in re_define.captures_iter(content) {
        println!("{:?}", cap);
        macros.push(Macro {
            name: cap[1].to_string(),
            args: cap[2].split(",").map(|i| i.to_owned()).collect(),
            expr: cap[3].to_string(),
            ast:  crate::ast::parse(&cap[3]).unwrap(),
        });
    }
    println!("{:#?}", macros);

    let mut source = re_define.replace_all(content, "").to_string();

    for mac in macros {
        let re = Regex::new(&format!(r#"(?m){}\(({})\)"#, mac.name, mac.args())).unwrap();
        source = re.replace(&source, |caps: &regex::Captures| {
            println!("{:?}", caps);
            let mut ast = mac.ast.clone();
            for (i, arg) in mac.args.iter().enumerate() {
                ast.replace_ident(arg, &caps[i + 1]);
            }
            ast.render()
        }).to_string();
    }

    Ok(source)
}
