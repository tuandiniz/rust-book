use std::io;
use std::cmp::Ordering::{Less, Greater, Equal};
use rand::Rng;
 
fn main() {
    println!("Guess the number!");

    let secret_number = rand::rng().random_range(1..=100);

    println!("The secret number is: {secret_number}");

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let guess: u32 = guess.trim().parse().expect("Please type a number!");

    match guess.cmp(&secret_number) {
        Less => println!("Too small!"),
        Greater => println!("Too big"),
        Equal => println!("You win!")
    }

    println!("You guessed: {guess}");
}
