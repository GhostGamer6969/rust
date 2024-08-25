use std::io::{self,Write};
use std::mem::size_of_val;
fn main() {
    let mut age=String::new();
    io::stdout().write(b"Enter your age: ").unwrap();
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut age).unwrap();
    let age:isize=age.trim().parse().unwrap();
    if age >= 18{
        println!("you can vote");
        print!("{}",size_of_val(&age));
    }
    else{
        println!("you cant vote");
        print!("{}",size_of_val(&age));
    }
}
