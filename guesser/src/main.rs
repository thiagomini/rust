use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number!");
    println!("Please input your guess.");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line!");

    println!("You guessed: {}", guess);

    let parsed_guess: u32 = guess.trim().parse().expect("Please type a number!");

    if parsed_guess == secret_number {
        println!("You win!");
    } else {
        println!("You lose! The number was {}.", secret_number);
    }
}
