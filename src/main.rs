use std::io::{self,Write};
fn main(){
    let mut f = String::new();
    io::stdout().write(b"Enter the number to factorial: ").unwrap();
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut f).unwrap();
    let f = f.trim().parse().unwrap();
    println!("{}",fact(f));
}

fn fact(f:i32)-> i32{
    if f == 0{
        1
    }else{
        f*fact(f-1)
    }
}
