use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Guess the number!");
        println!("Please input your guess.");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line!");

        println!("You guessed: {}", guess);
        let guess: u32 = guess.trim().parse().expect("Please type a number!");

        match guess.cmp(&secret_number) {
            Ordering::Less => print!("Too small!"),
            Ordering::Greater => print!("Too big!"),
            Ordering::Equal => {
                print!("You won!");
                break;
            }
        }
    }
}
