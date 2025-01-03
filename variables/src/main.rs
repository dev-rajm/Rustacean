fn main() {

    // 🙅‍♂️ Constants
    const MY_AGE: u8 = 21; 
    println!("My age is {MY_AGE}");

    // 👎 Immutable Variables
    let name = "Raj Manna";
    println!("My name is {name}");
    
    // 👍 Mutable Variables
    let mut total_marks = 466;
    println!("My total marks is {total_marks}");

    // 🥷 Shadowing the Variable
    let sum = 2; // Original Variable
    let sum = sum + 5; // Create a new sum variable which shadowed the original value(2) of sum and then adding 5 into it.

    // Block Level Shadowing(Only replace the pervious value in the block level)
    {
        let sum = sum * 2; // 7*2 = 14
        println!("The value of x in the inner scope is: {sum}"); // Verify the block level changes
    }

    println!("Value of x is {sum}"); // Global changes

}
