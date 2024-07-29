use std::io;
fn main() {
    let mut input = String::new();
    let n = io::stdin().read_line(&mut input).unwrap();
    println!("{} bytes read", n);
    println!("{}", input);
}
