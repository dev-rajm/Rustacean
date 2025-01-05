use std::io;
use rand::Rng;
use rand::prelude::SliceRandom;

fn main() {
    // Array of English Letters
    let letters = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z'];

    // Array of Numbers
    let numbers = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

    // Array of Symbols
    let symbols = ['!', '#', '$', '%', '&', '(', ')', '*', '+'];

    println!("Welcome to the Rust Password Generator 🙏");

    // Outer most loop labeled as parent
    'parent: loop {
        println!("How many letters would you like in your password 🔤?");

        let mut num_of_letters = String::new();    // To store the user input
    
        io::stdin()
        .read_line(&mut num_of_letters)
        .expect("Failed to read number of letters");    // User Input
    
        println!("How many symbols would you like 🔣?");
    
        let mut num_of_symbols = String::new();
    
        io::stdin()
        .read_line(&mut num_of_symbols)
        .expect("Failed to read number of symbols");
    
        println!("How many numbers would you like 🔢?");
    
        let mut num_of_numbers = String::new();
    
        io::stdin()
        .read_line(&mut num_of_numbers)
        .expect("Failed to read numbers");
        
        // Change the type to unsigned integer
        let num_of_letters: usize = match num_of_letters.trim().parse() {
            Ok(num) => num,
            Err(_) => continue 'parent
        };
        let num_of_symbols: usize = match num_of_symbols.trim().parse() {
            Ok(num) => num,
            Err(_) => continue 'parent
        };
        let num_of_numbers: usize = match num_of_numbers.trim().parse() {
            Ok(num) => num,
            Err(_) => continue 'parent
        };

        let mut rng = rand::thread_rng();    // Random number generator instance
        let mut password_list = Vec::new();    // Create a Vector to store the password items

        // Loop from 0 to array length
        for _ in 0..num_of_letters {
            let index = rng.gen_range(0..letters.len());    // Generate a random number between 0 to array length
            password_list.push(letters[index]);    // Push the items into the vector
        }

        for _ in 0..num_of_symbols {
            let index = rng.gen_range(0..symbols.len());
            password_list.push(symbols[index]);
        }

        for _ in 0..num_of_numbers {
            let index = rng.gen_range(0..numbers.len());
            password_list.push(numbers[index]);
        }

        // Shuffle the password
        password_list.shuffle(&mut rng);    // Shuffle the items 

        let password: String = password_list.iter().collect();    // Convert the vector into the string

        println!("Your password is ready 🤫: {password}");    // Output the final password

        break;    // Break the outer most loop
        
    }

}
