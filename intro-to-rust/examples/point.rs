struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn dist_from_origin(&self) -> f64 {
        (self.x * self.x + self.y * self.y)
            .sqrt()
    }
}

fn main() {
    let point = Point { x: 3.5, y: 4.0 };
    println!(
        "Distance from origin to ({}, {}) = {}",
        point.x,
        point.y,
        point.dist_from_origin()
    );
}
