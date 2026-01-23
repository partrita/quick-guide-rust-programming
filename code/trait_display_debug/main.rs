use std::fmt;

pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "POINT({} {})", self.x, self.y)
    }
}

impl fmt::Debug for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "POINT [{} {}]", self.x, self.y)
    }
}

fn main() {
    let dot: Point = Point { x: 1.2, y: 3.4 };
    println!("{:?}", dot);
    println!("For your information: {}", dot);
}
