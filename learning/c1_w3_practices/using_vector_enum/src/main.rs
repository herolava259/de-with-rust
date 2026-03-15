
enum Shape {
    Circle(f64),
    Square(f64),
    Triangle(f64), // base and height
}

impl Shape {
    fn largest_shape(shapes: &Vec<Shape>) -> Option<&Shape> {
        shapes.iter().max_by(|a, b| {
            let area_a = match a {
                Shape::Circle(radius) => std::f64::consts::PI * radius * radius,
                Shape::Square(side) => side * side,
                Shape::Triangle(base) => 0.5 * base * base, // Assuming an equilateral triangle for simplicity
            };
            let area_b = match b {
                Shape::Circle(radius) => std::f64::consts::PI * radius * radius,
                Shape::Square(side) => side * side,
                Shape::Triangle(base) => 0.5 * base * base, // Assuming an equilateral triangle for simplicity
            };
            area_a.partial_cmp(&area_b).unwrap()
        })
    }   
}

fn main() {
    let shapes = vec![Shape::Circle(5.0), Shape::Square(3.0), Shape::Triangle(4.0)];

    let total_areas: f64 = shapes
                        .iter()
                        .map(|shape| match shape {
                            Shape::Circle(radius) => std::f64::consts::PI * radius * radius,
                            Shape::Square(side) => side * side, 
                            Shape::Triangle(base) => 0.5 * base * base, // Assuming an equilateral triangle for simplicity  
                        })
                        .sum();
    
    println!("Total area: {}", total_areas);

    // example of finding the largest shape
    if let Some(largest) = Shape::largest_shape(&shapes){
        match largest {
            Shape::Circle(radius) => println!("Largest shape is a Circle with radius: {}", radius),
            Shape::Square(side) => println!("Largest shape is a Square with side: {}", side),
            Shape::Triangle(base) => println!("Largest shape is a Triangle with base: {}", base),
        }
    }
    else {
        println!("No shapes found.");
    }

}
