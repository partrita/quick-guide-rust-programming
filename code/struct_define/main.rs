struct Point {
    x: f32,
    y: f32,
}

struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

impl Rectangle {
    fn area(&self) -> f32 {
        let width = f32::abs(self.top_left.x - self.bottom_right.x);
        let height = (self.top_left.y - self.bottom_right.y).abs();
        width * height
    }
}

fn main() {
    let point1: Point = Point { x: 10.3, y: 0.4 };
    let point2: Point = Point { x: 22.5, y: 2.4 };
    let rect = Rectangle {
        top_left: point1,
        bottom_right: point2,
    };
    println!("area size={}", rect.area());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_struct_define() {
        let point1: Point = Point { x: 10.3, y: 0.4 };
        let point2: Point = Point { x: 22.5, y: 2.4 };
        let rect = Rectangle {
            top_left: point1,
            bottom_right: point2,
        };
        println!("area size={}", rect.area());
    }
}
