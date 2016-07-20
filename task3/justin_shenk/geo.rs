// Aufgabe 3: Mehr Rust Aufgaben
// Points and Rectangles

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

    pub fn distance(p1: Point, p2: Point) -> f32 {
        ((p1.x.abs() - p2.x.abs()).powi(2) + (p1.y.abs() - p2.y.abs()).powi(2)).sqrt()
    }

    pub fn is_origin(&self) -> bool {
        self.x == 0.0 && self.y == 0.0
    }
}
struct Rectangle {
    p1: Point,
    p2: Point,
}

impl Rectangle {
    pub fn new(p1: f32, p2: f32) -> Rectangle {
        Rectangle { p1: p1, p2: p2 }
    }

    pub fn area(&self) -> f32 {
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;
            return (x1.abs() - x2.abs()).abs() * (y1.abs() - y2.abs()).abs();
        }

    pub fn contains(&self, p: &Point) {
        let Point { x: x, y: y } = p;
        x > self.p1.x && x < self.p2.x && y > self.p1.y && y < self.p2.y
    }
}
