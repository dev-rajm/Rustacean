fn main() {
    // Data Types
    let num: u8 = 32; // Integer (8-bit Unsigned)
    println!("Value of num is {num}");

    let pi = 3.14; // Float (64-bit)
    println!("Value of Pi is {pi}");

    // Numeric Operations
    let sum = 5+10;
    println!("5 + 10 = {sum}");

    let difference = 32.2 - 12.5;
    println!("32.2 - 12.5 = {difference}");

    let product = 3*2;
    println!("3 x 2 = {product}");

    let division = -5/3;
    println!("-5 / 3 = {division}");

    let modulo = 43%5;
    println!("43 % 5 = {modulo}");

    // let fs: bool = false; // Boolean

    let emoji: char = '✨'; // Character
    println!("Emoji: {emoji}");

    let tup: (i32, f64, u8) = (500, 40.5, 1); // Tuple
    let (money, age, _sib) = tup; // Destructure the tuple
    println!("I have {money}$ at the age of {age}."); // Access as a variable
    println!("I have {} sister.", tup.2); // Access from tup by index

    let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"]; // Array
    let third_month = months[2];
    println!("Third Month is: {third_month}");

    let i: [i32; 5] = [1, 2, 3, 4, 5]; // [type; length]
    let start = i[4];
    println!("Start Counting from {start}");

    let zero = [0; 5]; // [element; length] => [0, 0, 0, 0, 0]
    let items = zero[0];
    println!("Zero Array Contains: {items}");


}
