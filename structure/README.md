# Struct in Rust

Structs let you structure data together (similar to struct in C/C++)

## Example

```rust
// Group user field in a single User structure
struct User {
    first_name: String,
    last_name: String,
    age: u32
}

fn main() {
    let user = User {
        first_name: String::from("Raj"),
        last_name: String::from("Manna"),
        age: 69
    };

    // Access the data from structure
    println!("First name is {}", user.first_name);
}
```