
fn main() {
    for x in 'a'..='z' {
        println!("{:b}", x as u16); // :o - octal and :x - hex
    }
    println!("Success!");
}
