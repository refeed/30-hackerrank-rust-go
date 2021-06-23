use std::io;

fn main() {
    let mut input = String::new();

    io::stdin()
    .read_line(&mut input)
    .expect("Error when reading the line");

    println!("Hello, World.");
    println!("{}", input)
}
