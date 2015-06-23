#![allow(dead_code)]

mod list;
mod parse;

use parse::Lexer;

fn main() {
    let instr = r#"(this is (a 42 #t "list"))"#;
    for val in Lexer::new(instr).parse() {
        println!("{}", val);
    }
}
