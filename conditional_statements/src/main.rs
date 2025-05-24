// 2. Write a function `fib` that finds the fibonacci of a number it takes from input

fn main() {
    println!("{}", fib_l(10));
}

// using recursion
fn fib_r(num: u32) ->u32 {
    if num<=1 {
        return num;
    }

    return fib_r(num-1) + fib_r(num-2);
}

// using loop
fn fib_l(num: u32) -> u32 {
    let mut first = 0;
    let mut second = 1;

    if num==0 {
        return first;
    }
    if num==1 {
        return second;
    }

    for _ in 0..num-1 {
        let temp = second;
        second = first+second;
        first=temp;
    }

    return second;
}