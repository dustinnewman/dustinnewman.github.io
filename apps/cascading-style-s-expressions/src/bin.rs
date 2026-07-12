use std::{env, path::Path, fs};

use css::{lex, parse};

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = Path::new(&args[1]);
    let input = fs::read_to_string(path).unwrap();
    if let Ok(tokens) = lex(input) {
        let s_exprs = parse(tokens);
        println!("{:?}", s_exprs);
        for s_expr in s_exprs {
            print!("{}", s_expr.to_stylesheet(""));
        }
    }
}