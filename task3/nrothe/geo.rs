fn main() {
    let p = Point::origin();
    println!("{}", p.is_origin());
    let q = Point::new(5.0, 0.0);
    println!("{}", q.is_origin());
    println!("{}", q.distance(p));
}

struct Point {
    x: f32,
    y: f32,
}

impl Point {
    pub fn origin() -> Point {
        Point { x: 0.0, y: 0.0 }
    }

    pub fn new(a: f32, b: f32) -> Point {
        Point { x: a, y: b }
    }

    pub fn is_origin(&self) -> bool {
        if self.x == 0.0 && self.y == 0.0
    }

    pub fn distance(&self, p: Point) -> f32 {
        (((self.x - p.x) * (self.x - p.x)) + ((self.y - p.y) * (self.y - p.y))).sqrt()
    }
}

struct Rectangle{
    p1: f32,
    p2: f32,
    length: f32,
    width: f32,
}
