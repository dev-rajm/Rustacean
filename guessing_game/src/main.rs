use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Welcome to the Number Guessing Game!");
    println!("I'm thinking of a number between 1 and 100. Can you guess it?");

    let secret_number = rand::thread_rng().gen_range(1..=100); // Generate a integer number between 1-100
    let mut chances = 10;

    loop {
        println!("Please enter your guess: ");
    
        let mut guess = String::new();
    
        io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line.");
    
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => {
                println!("Too small! Try again.");
                chances -= 1;
            },

            Ordering::Greater => {
                println!("Too big! Try again");
                chances -= 1;
            }

            Ordering::Equal => {
                println!("Congratulations! You guessed the number in {} attempts.", 11-chances);
                break;
            }
        }

        if chances==0 {
            println!("Out of chances! The secret number was {secret_number}. Better luck next time.");
            break;
        }
    }


}