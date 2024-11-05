// enums: enums are used to create custom data types just like structs. The difference between enums and structs is that enums can have multiple data types in a single data type.
// Common use cases of enums are:
//    * To define a type that can have one of a few variants
//    * To define a type that can have different types of data
//    * To define a type that can have different types of data with different variants
// Common enum types are:
//    * Option<T>
//    * Result<T, E>
//    * std::io::Result<T>
//    * std::io::Error
//    * std::net::IpAddr

pub enum Shape {
    Circle(f64),
    Square(f64),
    Rectangle(f64, f64),
    Triangle(f64, f64),
}

pub fn calculate_area(shape: Shape) -> f64{
    let result = match shape {
        Shape::Circle(radius) => std::f64::consts::PI * radius * radius,
        Shape::Square(side) => side * side,
        Shape::Rectangle(length, breadth) => length * breadth,
        Shape::Triangle(base, height) => 0.5 * base * height,
    };

    return result
}

pub fn plus_one(x: Option<i32>) -> Option<i32> {
    let result = match x {
        None => None,
        Some(i) => Some(i + 1),
    };

    return result;
}
