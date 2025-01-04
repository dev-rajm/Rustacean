fn main() {

    let x = 5; // Statement

    let sum = add_y(x); // Statement

    println!("X+Y = {sum}");

    another_function(5.6,  'f'); // Call the function
}

fn add_y(num: i32) -> i32 {
    let y = 9;
    num + y // Expression (basically its return the num+y)
}

fn another_function(height: f32, unit: char) {
    println!("My height is {height} {unit}");
}
