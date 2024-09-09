use ::rand::Rng;
use ::std::io;
use std::cmp::Ordering;
fn main() {
    println!("Guess the number !");
    let secret_number = guesser();
    loop {
        println!("Enter you guess");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess) // Here we need a mutable reference because the user input shall be written inside the guess variable, so we need a mutable reference to manipulate its value
            .expect("Failed to read line for guessed number");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid numer");
                continue;
            }
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Small"),
            Ordering::Greater => println!("Too Big"),
            Ordering::Equal => {
                println!("Got it, you win !");
                break;
            }
        }
    }
}

fn guesser() -> u32 {
    rand::thread_rng().gen_range(1..=100)
}
