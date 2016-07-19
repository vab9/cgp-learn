fn main() {
    let a = Point::new(4.5, 2.5);
    let b = Point::origin();
    println!("dist: {}", Point::distance(a, b));

    let rect = Rectangle::new(0.0, 0.0, 10.0, 10.0);
    println!("contains a: {}", rect.contains(a));
    println!("contains b: {}", rect.contains(b));
}

#[derive(Copy, Clone)]
pub struct Point {
    x: f32,
    y: f32,
}

impl Point {
    pub fn new(x: f32, y: f32) -> Point {
        Point {
            x: x,
            y: y,
        }
    }

    pub fn origin() -> Point {
        Point::new(0.0, 0.0)
    }

    pub fn is_origin(&self) -> bool {
        self.x == 0.0 && self.y == 0.0
    }

    pub fn distance(a: Point, b: Point) -> f32 {
        let dx = a.x - b.x;
        let dy = a.y - b.y;

        (dx * dx + dy * dy).sqrt()
    }
}

pub struct Rectangle {
    top_left: Point,
    width: f32,
    height: f32,
}

impl Rectangle {
    pub fn new(x: f32, y: f32, w: f32, h: f32) -> Self {
        Rectangle {
            top_left: Point::new(x, y),
            width: w,
            height: h,
        }
    }

    pub fn area(&self) -> f32 {
        self.width * self.height
    }

    pub fn contains(&self, p: Point) -> bool {
        p.x >= self.top_left.x &&
        p.y >= self.top_left.y &&
        p.x <= self.top_left.x + self.width &&
        p.y <= self.top_left.y + self.height
    }
}
