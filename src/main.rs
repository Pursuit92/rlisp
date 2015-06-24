#![allow(dead_code)]

mod parse;
mod val;

use parse::Parser;

fn main() {
    let instr = r#"(this is '(a 42 #t "list"))"#;
    let parser = Parser::new();
    for val in parser.parse(instr) {
        println!("{}", val);
    }
    for val in parser.parse(instr) {
        println!("{}", val);
    }
}
