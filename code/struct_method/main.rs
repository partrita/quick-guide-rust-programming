#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

#[derive(Debug)]
struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

impl Rectangle {
    fn new() -> Rectangle {
        Rectangle {
            top_left: Point { x: 0.0, y: 0.0 },
            bottom_right: Point { x: 0.0, y: 0.0 },
        }
    }

    fn area(&self) -> f32 {
        let width = f32::abs(self.top_left.x - self.bottom_right.x);
        let height = (self.top_left.y - self.bottom_right.y).abs();
        width * height
    }

    fn destroy(self) {
        // do nothing but free myself
        println!("destroyer");
    }
}

fn main() {
    let rect = Rectangle::new();

    {
        let point1: Point = Point { x: 10.3, y: 0.4 };
        let point2: Point = Point { x: 22.5, y: 2.4 };
        let rect2 = Rectangle {
            top_left: point1,
            bottom_right: point2,
        };
        rect2.destroy();

        //println!("area size={} {:?}", rect2.area(), rect2); // compile error!!!
    }

    println!("area size={} {:?}", rect.area(), rect);
}
