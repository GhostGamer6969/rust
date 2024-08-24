use std::io::{self, Write};
fn main() {
    let mut name=String::new();
    io::stdout().write(b"enter name: ").unwrap();
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut name).unwrap();
    println!("Hello, {}!", name.trim());
    println!("Success!");
}
