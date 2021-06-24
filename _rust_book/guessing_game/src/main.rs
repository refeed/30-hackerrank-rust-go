use rand::Rng;
use std::io;
use std::cmp::Ordering;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..100);

    println!("Guess the number!");

    let mut number_of_guesses = 0;

    loop {
        number_of_guesses += 1;

        println!("Please enter your guess");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Error when reading the line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win! answer: {}, guesses: {}",
                         secret_number, number_of_guesses);
                break;
            },
        };
    }
}
