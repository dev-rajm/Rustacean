// struct in rust is more like a class in javascript
struct Rect {
    width: i32,
    height: i32,
}

// implement some method on the top of `Rect` struct
impl Rect {
    // Similar to static method that can call over on struct not on object
    fn debug() -> i32 {
        return 1;
    }

    // method that calculate the area of given rectangle
    fn area(&self) -> i32 {
        self.width * self.height
    }

    // method that calculate the perimeter of given rectangle with accepting a arg `num`
    fn perimeter(&self, num: i32) -> i32 {
        2 * (self.width + self.height)
    }
}

fn main() {
    let rect1 = Rect {
        width: 10,
        height: 20,
    };

    println!("Debug is {}", Rect::debug());
    println!("Area is {} m^2", rect1.area());
    println!("Perimeter is {} m", rect1.perimeter(1));
}