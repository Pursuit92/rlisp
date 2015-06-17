mod list;
mod lispval;

fn main() {
    println!("{}",list::List::new().push(4).push(5).append(1).reverse());
    let mut lexer = lispval::Lexer::new(r#"(this is (a "list"))"#);
    for tok in lexer.parse() {
        println!("{:?}", tok);
    }

}
