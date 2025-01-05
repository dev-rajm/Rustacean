# Rust Password Generator 🙌

A simple and secure password generator written in Rust. This program allows users to create strong, randomized passwords by specifying the number of letters, symbols, and numbers they want in the password.

## 🌟 Features

- Generates passwords with user-defined composition.
- Supports:
  - Uppercase and lowercase letters.
  - Numbers (0-9).
  - Special symbols (!, #, $, %, etc.).
- Randomized character order for added security.
- Simple and interactive command-line interface

## ⚙️ How It Works

1. The program asks the user for:
   - Number of letters.
   - Number of symbols.
   - Number of numbers.
2. It generates random characters based on the user's input.
3. The characters are shuffled to randomize the password.
4. The final password is displayed on the screen.

## 🎈 Example Output

```
Welcome to the Rust Password Generator 🙏
How many letters would you like in your password 🔤?
5
How many symbols would you like 🔣?
3
How many numbers would you like 🔢?
4
Your password is ready 🤫: zA#2B!8m*5
```

## 📜 Code Overview

### Core Functionalities

- **Random Character Generation**: Uses the rand crate to randomly pick characters from predefined arrays (`letters`, `numbers`, `symbols`).

- **Password Composition**: Combines user-specified counts of letters, numbers, and symbols into a single password.

- **Shuffling**: Ensures the order of characters is randomized for better security.

### Key Code Snippets

#### Generating Random Characters

```rust
let idx = rng.gen_range(0..letters.len());
password.push(letters[idx]);
```

#### Shuffling the Password

```rust
password_list.shuffle(&mut rng);
```

#### Collecting and Displaying

```rust
let password: String = password.iter().collect();
println!("Your generated password is: {password}");
```

## How to Run?

```bash
cd ./Rustacean/loops

cargo run
```

Happy coding! 🚀
