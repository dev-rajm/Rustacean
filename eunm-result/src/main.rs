// 5. Write a function that read contents from a file

use std::fs::read_to_string;

fn main() {
   let result = read_to_string("a.txt"); // returned a Result<String, Error>

   // pattern matching
   match result {
    Ok(data) => println!("{}", data),
    Err(_err) => println!("Error while reading file") // Return an error if file is not exist or some other error occurred
    // Err(err) => panic!("File not found") // Stop executing the program if error occurred
   }
}
