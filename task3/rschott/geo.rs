fn main() {
    let p = Point::new(3.14, 4.56);
    let q = Point::origin();
    println!("{}", p.is_origin());
    println!("{}", q.is_origin());
    println!("Distance: {}", p.distance(q));
}

struct Point {
    x: f32,
    y: f32,
}

impl Point {
    pub fn new(a: f32, b: f32) -> Point {
        Point { x: a, y: b }
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
    pub fn distance(&self, q: Point) -> f32 {
        ((q.x - self.x) * (q.x - self.x) + (q.y - self.y) * (q.y - self.y)).sqrt()
    }
}
