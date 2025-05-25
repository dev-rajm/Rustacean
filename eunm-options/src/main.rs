// 4. Write a function that returns the index of the first "a" in a string
// Efficient way
fn main() {
    let index = get_first_a(String::from("Swarnali"));
    
    match index {
        Some(val) => println!("index: {}", val),
        None => println!("a not found")
    }

}

fn get_first_a(str: String) -> Option<i32> {
    for(index, char) in str.chars().enumerate() {
        if char == 'a' {
            return Some(index as i32);
        }
    }

    return None;
}

// Normal way
/* 
fn main() {
    let index = get_first_a(String::from("Swarnali"));

    if index == -1 {
        println!("a not found");
    } else {
        println!("index: {}", index);
    }

}

fn get_first_a(str: String) -> i32 {
    for(index, char) in str.chars().enumerate() {
        if char == 'a' {
            return index as i32;
        }
    }
    return -1;
}
*/

