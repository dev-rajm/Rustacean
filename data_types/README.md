# 📚 Data Types in Rust

This README explains various data types in Rust with examples from a simple program. Rust is a statically typed language, meaning every value has a type known at compile time. Let's explore!

## 🛠️ Code Overview

The program demonstrates:

- **Primitive Types**: Integers, floats, booleans, and characters.
- **Compound Types**: Tuples and arrays.
- **Basic Operations**: Arithmetic with numeric types.

## How to Run?

```bash
cd ./Rustacean/datatypes

cargo run
```

## 🌟 Key Concepts

Please refer to Official Rust Book for more types and examples.

```bash
rustup doc --book    # Open Rust Book using terminal/bash
```

### 1. Primitive Data Types

#### INTEGER

```rust
let age: u8 = 32;    // Unsigned integer
let marks: i32 = 377;    // Signed integer

println!("My age is {age}, I passed with {marks} marks.");
```

#### Float

```rust
let pi = 3.14;    // By default 64-bit in size
println!("Value of Pi is {pi}");
```

#### Boolean

```rust
let fs: bool = false;    // Boolean values: true or false
```

#### Character

```rust
let emoji: char = '🗿';    // Can be anything letter, emojis, ASCII
println!("My favorite emoji is {emoji}");
```

### 2. Compound Types

#### Tuple

```rust
let tup: (i32, f64, u8) = (500, 40.5, 1);     // Tuple with mixed types
let (money, age, _sib) = tup;     // Destructuring

println!("I have {money}$ at the age of {age}.");    // Access as variables
println!("I have {} sister.", tup.2);    // Access by index
```

#### Array

```rust
let months = ["January", "February", "March", "April", "May"];    // Array of strings
let third_month = months[2];    // Access by index

println!("Third Month is: {third_month}");

let i: [i32; 5] = [1, 2, 3, 4, 5]; // Array of Singed Integers of size 5

let zero = [0; 5]; // Array of 5 zeros

```

### 3. Numeric Operations

```rust
let sum = 5 + 10; // Addition
let difference = 32.2 - 12.5; // Subtraction
let product = 3 * 2; // Multiplication
let division = -5 / 3; // Division (integer division for integers)
let modulo = 43 % 5; // Modulus
```

## 🔗 Key Takeaways

- Rust ensures type safety and checks for mismatched types at compile time.
- Compound types like `tuples` and `arrays` help group multiple values.
- Arithmetic operations are straightforward and intuitive.
- Rust supports Unicode characters for `char`.

Happy coding with Rust! 🚀
