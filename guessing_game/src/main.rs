use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100); // Generate a integer number between 1-100
    let mut chances = 5;

    loop {
        println!("Chances left: {chances}!");
        println!("Please enter your guess!");
    
        let mut guess = String::new();
    
        io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => {
                println!("Too small!");
                chances -= 1;
            },

            Ordering::Greater => {
                println!("Too big!");
                chances -= 1;
            }

            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }

        if chances==0 {
            println!("Out of chances! The secret number was {secret_number}. Better luck next time.");
            break;
        }
    }


}