use std::io;

fn main() {
    let mut first_number = String::new();
    let mut second_number = String::new();
    let mut third_string = String::new();

    io::stdin()
        .read_line(&mut first_number)
        .expect("Error when parsing");
    io::stdin()
        .read_line(&mut second_number)
        .expect("Error when parsing");
    io::stdin()
        .read_line(&mut third_string)
        .expect("Error when parsing");

    let first_number: i32 = first_number.trim().parse::<i32>()
        .expect("Error when parsing") + 4;
    let second_number: f32 = second_number.trim().parse::<f32>()
        .expect("Error when parsing") + 4.0;

    third_string = String::from("HackerRank ") + &third_string;

    println!("\n{}\n{:.1}\n{}", first_number, second_number, third_string);
}
