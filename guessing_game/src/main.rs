use rand::Rng;
use std::io;

fn main() {
    println!("Guess the number between 1 and 100 (and maybe win a prize)!");
    println!("Please input your guess:");

    let mut guess = String::new();

    let secret_number = rand::thread_rng().gen_range(1..=100);

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read the user input line");
    
    println!("You guessed: {guess}");
    println!("The universe guessed: {secret_number}");
}
