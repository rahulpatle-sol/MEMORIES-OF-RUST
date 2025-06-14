fn main() {
    
    let circle = Shape::Circle(5.0);
    let rectangle = Shape::Ractangle(4.0, 6.0);
    let triangle = Shape::Triaangle(3.0, 4.0);
println!("Area of Circle: {}", _calculate_area(circle));
    println!("Area of Rectangle: {}", _calculate_area(rectangle));
    println!("Area of Triangle: {}", _calculate_area(triangle));
}
//  enum  in  Rust is like a class in JavaScript
// pattern  matching is used to calculate area of different shapes
enum Shape{
    Circle(f64),
    Ractangle(f64, f64),
    Triaangle(  f64, f64),
}

fn _calculate_area(shape:Shape)-> f64 {
    let area:f64 = match shape {
        Shape::Circle(radius) => std::f64::consts::PI * radius * radius,
        Shape::Ractangle(length, width) => length * width,
        Shape::Triaangle(base, height) => 0.5 * base * height,
    };
return area;
}