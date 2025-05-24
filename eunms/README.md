# Enums in Rust

Enums let you enumerate over various types of value (similar we have in typescript). When you have limited values to give access, use `enum`.

## Example

We use enum to this because direction should be one of the four values, so you are restricted the program to use one of four values from Direction.

```rust
// Define Enum Direction
enum Direction {
    North,
    South,
    East,
    West
}

fn main() {
    let my_direction = Direction::North; // valid ✅
    let her_direction = Direction::Cheating; // invalid ❎
    move_around(my_direction);
}

fn move_around(direction: Direction) {
    // Make your logic accordingly
}
```