mod list;
mod lispval;

use list::OwnedIterator;

fn main() {
    let l = list::List::new().push(4).push(5).append(1).reverse();
    let mut it = l.iter();
    loop {
        match it.next_owned() {
            None => break,
            _ => println!("stuff"),
        }
    }
}
