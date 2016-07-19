#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

impl Point {
    pub fn new(x: f32, y: f32) -> Point {
        Point { x: x, y: y }
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
    pub fn distance(p1: &Point, p2: &Point) -> f32 {
        ((p2.x - p1.x).powf(2.0) + (p2.y - p1.y).powf(2.0)).sqrt()
    }
}

struct Rectangle {
    p1: Point,
    width: f32,
    height: f32,
}

impl Rectangle {
    pub fn new() -> Rectangle {
        Rectangle {
            p1: Point { x: 0.0, y: 0.0 },
            width: 1.0,
            height: 1.0,
        }
    }
    pub fn area(&self) -> f32 {
        (self.p1.x + self.width * self.p1.y + self.height).abs()
    }
    pub fn contains(&self, point: Point) -> bool {
        if point.x >= self.p1.x && point.x <= self.p1.x + self.width && point.y >= self.p1.y &&
           point.y <= self.p1.y + self.height {
            true
        } else {
            false
        }
    }
}

fn main() {
    let p = Point::new(1.0, 1.0);
    println!("{:?}", Point::origin());
    println!("{:?}", p.is_origin());
    println!("{:?}", Point::distance(&p, &Point::origin()));
    let r = Rectangle::new();
    println!("{:?}", r.area());
    println!("{:?}", r.contains(p));
}
