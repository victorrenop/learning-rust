use rand::Rng;
use std::{io, cmp::Ordering};

fn main() {
    println!("Guess the number between 1 and 100 (and maybe win a prize)!");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    loop {
        println!("Please input your guess:");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read the user input line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        
        println!("You guessed: {guess}");
        
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("I expected more from you..."),
            Ordering::Greater => println!("Lower your expectations please"),
            Ordering::Equal => {
                println!("You are correct... for now");
                break;
            },
        }
    }
}
