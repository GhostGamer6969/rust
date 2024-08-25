use std::io::{self};

fn add(n1:f32,n2:f32)->f32{
    n1+n2
}

fn subt(n1:f32,n2:f32)->f32{
    n1-n2
}

fn mul(n1:f32,n2:f32)->f32{
    n1*n2
}

fn div(n1:f32,n2:f32)->f32{
    n1/n2
}

fn main(){
    let (mut n1,mut n2,mut s)=(String::new(),String::new(),String::new());

    println!("Enter a num: ");
    io::stdin().read_line(&mut n1).expect("Failed to read line");
    let inp1:f32=n1.trim().parse().expect("Enter valid number");

    println!("Enter a num: ");
    io::stdin().read_line(&mut n2).expect("Failed to read line");
    let inp2:f32=n2.trim().parse().expect("Enter valid number");

    println!("Enter a operation: ");
    io::stdin().read_line(&mut s).expect("Failed to read line");
    let opr=s.trim();

    let result = match opr{
        "+"=>add(inp1,inp2),
        "-"=>subt(inp1,inp2),
        "*"=>mul(inp1,inp2),
        "/"=>div(inp1,inp2),
        _=>{
            println!("Invalid operation");
            return;
        }
    };
    println!("result {}",result);
}
