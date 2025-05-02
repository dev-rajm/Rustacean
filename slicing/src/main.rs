fn main() {
    let s = String::from("Hello World!"); // String allocated on heap

    let word = first_word(&s[0..6]); // Hello
    println!("the first word is: {word}");

    let word = first_word(&s[..]); // whole string by reference
    println!("the first word is: {word}");

    let word = first_word(&s); // whole string by reference
    println!("the first word is: {word}");

    let my_s = "hello world"; // str is a string slice

    let word = first_word(&my_s[0..6]); // hello
    println!("the first word is: {word}");

    let word = first_word(&my_s[..]); // whole string by address
    println!("the first word is: {word}");

    let word = first_word(my_s); // whole string by value
    println!("the first word is: {word}");
}

fn first_word(s: &str) -> &str {
    let bytes  = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

// problem: write a function that takes a string of words separated by spaces and returns the first word it finds in that string. If the function doesn’t find a space in the string, the whole string must be one word, so the entire string should be returned.