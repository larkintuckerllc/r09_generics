fn main() {
    // IN FUNCTION
    let x = identity(0); // i32
    let y = identity(3.0); // f64

    // IN STRUCT
    let integer = Point { x: 5, y: 10 }; // Point<i32>
    let float = Point { x: 1.0, y: 4.0 }; // Point<f64>

    // IN METHOD
    let integer_x = integer.x(); // &i32
    let float_x = float.x(); // &f64

    // IN METHOD SPECIFIC TYPE
    let d = float.distance_from_origin(); // f64
}

fn identity<T>(item: T) -> T {
    item
}

struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f64> {
    fn distance_from_origin(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}
