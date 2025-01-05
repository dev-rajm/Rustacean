use std::io;

fn main() {
    let mut age = String::new(); // Mutable variable age

    println!("Enter you age!");

    // Take the age from user
    io::stdin()
    .read_line(&mut age)
    .expect("Failed to read age");

    let age: u8 = age.trim().parse().expect("Age must be a number"); // Typecast the age from string to number

    let check = if can_drive(age) {"You can drive 🥳"} else {"You can't drive at this age 👴"}; // If-Else inline/in let statement

    println!("Report: {check}");
}

// Can I drive?
fn can_drive(age: u8) -> bool {
    // Check if age is less than 18y
    if age < 18 {
        false
    } 
    // Check if age is greater than 48y
    else if age > 48 {
        false
    } 
    // Check if age is between 18 and 48
    else {
        true
    }
}
