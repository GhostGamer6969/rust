fn main() {
    let (mut x,y) = (1,2);
    x += 2;
    {
        x = 1;
        assert_eq!(x, 1);
        println!("x is {}", x);
    }
    assert_eq!(y, 2);

    println!("Success!");
}
