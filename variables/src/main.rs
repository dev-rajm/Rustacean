// 1. Write a function `is_even` that takes a number as an input and return true if even, false otherwise

fn main() {
    let ans = is_even(13);
    println!("{}", ans);
}

fn is_even(num: i32) -> bool {
    if num % 2 == 0 {
        return true;
    }
    return false;
}
