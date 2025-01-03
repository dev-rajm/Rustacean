# Number Guessing Game 🎮

A simple command-line-based number guessing game built using Rust. In this game, the user attempts to guess a randomly generated number within a range of 0-100. It's a fun way to practice Rust programming and interact with the terminal.

## ✨ Features

- Generate a random integer within a range of 0-100.
- Provides feedback on whether the guess is too small or too big.
- Tracks the number of attempts made by the user.
- Simple and interactive CLI interface.

## ▶️ How to Play

1. The program will generate a random number with a defined range (1-100).
2. Enter your guess in the terminal.
3. The program will inform you if your guess is too big, too small or correct.
4. Keep guessing until you have enough chances (5 times).
5. If you guess the correct number then you win or if you burn your all chances you loss the game.
6. Correct number will be displayed at the end.

## ⬇️ Installation

1. Ensure you have Rust installed on your system.

2. Clone this repo:

```bash
git clone https://github.com/rajmanna-dev/Rustacean.git

cd Rustacean/guessing_game/
```

3. Build and run the game:

```bash
cargo build

cargo run
```

## 🕹️ Example GamePlay

```
Welcome to the Number Guessing Game!
I'm thinking of a number between 1 and 100. Can you guess it?

Enter your guess: 50
Too high! Try again.

Enter your guess: 25
Too low! Try again.

Enter your guess: 37
Congratulations! You guessed correct.
```

## 🧩 Customization

1. You can modify the range of numbers by editing the range in the source code. Look for this line in `src/main.rs`:

```rust
let secret_number = rand::thread_rng().gen_range(1..=100); // Create a immutable variable secret_number and bind it with randomly generated number
```

Change `1..=100` to your desired range.

2. You can change the number of chances in `src/main.rs`:

```rust
let mut chances = 5; // Create a mutable variable chances and bind it with the number of chances.
```

Change `5` to your desired number(integer).

## 📖 Learning Outcomes

- Learn basic Rust concepts like variables, loops and match conditions.
- Get familiar with handling the user input and random number generation.
- Practice building CLI programs with Rust.

Feel free to adjust the details (e.g., GitHub URL, customization instructions) to fit your specific implementation.
