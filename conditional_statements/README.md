# 🚗 Can You Drive? - Rust Conditional Statements

This is a simple Rust program that determines if a person is eligible to drive based on their age. The program uses conditional statements to evaluate the eligibility criteria and demonstrates how to take user input, parse it, and use functions effectively.

## 🛠️ Features

- **User Input**: Accepts the user's age via the console.
- **Conditional Logic**: Uses if, else if, and else statements to evaluate driving eligibility.
- **Function Usage**: Implements a separate function, can_drive, to encapsulate the logic for determining eligibility.
- **Inline Conditional**: Demonstrates the use of if-else within a let statement for concise output.

## 📝 How It Works

1. **Prompt for Input**: The program prompts the user to enter their age.
2. **Input Parsing**: The input is read as a string and converted to an unsigned integer (`u8` range from 0 to 255).
3. **Conditional Evaluation**: The can_drive function checks if the age falls within the allowed range (18–48 years).
4. **Output Result**: Based on the evaluation, a message is displayed to indicate whether the user can drive.

## 🚀 How to Run

```bash
cd ./Rustacean/can_drive

cargo run
```

## 🔗 Example Usage

```terminal
Enter your age!
25
Report: You can drive 🥳
```

```terminal
Enter your age!
50
Report: You can't drive at this age 👴
```

## 📚 Learning Highlights

- Use of conditional statements (`if`, `else if`, `else`) in Rust.
- Reading and parsing user input.
- Structuring code with functions for modularity.
- Inline conditionals for concise logic representation.

## 🏆 Happy Coding with Rust!

Feel free to add more sections or personalize this as needed! 😊
