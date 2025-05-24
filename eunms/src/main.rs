// Enum with values
enum Shape {
    Rectangle(f64, f64), // Rectangle(side1, side2)
    Circle(f64), // Circle(radius)
    Square(f64), // Square(side)
}

fn main() {
    let circle = Shape::Circle(5.0);
    println!("Area of the circle is {}", calculate_area(circle));
    
    let square = Shape::Square(4.0);
    println!("Area of the square is {}", calculate_area(square));

    let rectangle = Shape::Rectangle(3.0, 6.0);
    println!("Area of the rectangle is {}", calculate_area(rectangle));
}

fn calculate_area(shape: Shape) -> f64 {
    // pattern matching
    match shape {
        Shape::Rectangle(a, b) => a * b,
        Shape::Circle(a) => 3.14 * a * a, 
        Shape::Square(a) => a * a,
    }
}

