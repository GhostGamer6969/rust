use std::io::{self,Write};
fn main() {
    let mut age=String::new();
    io::stdout().write(b"Enter your age: ").unwrap();
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut age).unwrap();
    let age:i32=age.trim().parse().unwrap();
    if age >= 18{
        println!("you can vote");
    }
    else{
        println!("you cant vote");
    }
}
