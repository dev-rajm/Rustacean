// 3. Write a function `get_string_length` that takes a string as an input and returns its length

fn main() {
    let name = String::from("raj"); // Create a string
    let len = get_string_length(name);
    println!("The length of the string is {}", len);
}

fn get_string_length(str: String) -> usize {
    str.chars().count() // get the length
}
