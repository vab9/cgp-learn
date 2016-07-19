fn main() {
    let p1 = Point::origin();
    let p2 = Point::new(1.0, 1.0);

    println!("{}", p1.is_origin());
    println!("{}", p2.is_origin());
    println!("{}", Point::distance(p1, p2));
}

struct Point {
    x: f32,
    y: f32,
}

impl Point {
    pub fn new(in_x: f32, in_y: f32) -> Point {
        Point { x: in_x, y: in_y }
    }

    pub fn origin() -> Point {
        Point { x: 0.0, y: 0.0 }
    }

    pub fn is_origin(&self) -> bool {
        if self.x == 0.0 && self.y == 0.0 {
            true
        } else {
            false
        }
    }

    pub fn distance(p1: Point, p2: Point) -> f32 {
        ((p1.x - p2.x) * (p1.x - p2.x) + (p1.y - p2.y) * (p1.y - p2.y)).sqrt()
    }
}
