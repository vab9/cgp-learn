fn main() {
    let p1 = Point::new(1.0, 1.0);
    let p2 = Point::origin();
    println!("Point ({},{}) is origin: {}", p2.x, p2.y, p2.is_origin());
    println!("Point 1 ({},{})", p1.x, p1.y);
    println!("Point 2({},{})", p2.x, p2.y);
    println!("Distance between points: {}", Point::distance(&p1, &p2));

    let r1 = Rectangle::new(p1, p2);
    println!("Rectangle Point 1 ({},{}) Point 2 ({},{})",
             r1.p1.x,
             r1.p1.y,
             r1.p2.x,
             r1.p2.y);
    println!("Area: {}", r1.area());

    let p3 = Point::new(0.5, 0.5);
    let p4 = Point::new(2.0, 2.0);

    println!("Point 3 ({},{})", p3.x, p3.y);
    println!("Point 4({},{})", p4.x, p4.y);

    println!("Point 3 contained in Rectangle: {}", r1.contains(&p3));
    println!("Point 4 contained in Rectangle: {}", r1.contains(&p4));
}

#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

impl Point {
    // New point with the given coordinates
    pub fn new(a: f32, b: f32) -> Point {
        Point { x: a, y: b }
    }
    // New origin point
    pub fn origin() -> Point {
        Point { x: 0.0, y: 0.0 }
    }
    // Tests wether a point lies in the origin
    pub fn is_origin(&self) -> bool {
        if self.x == 0.0 && self.y == 0.0 {
            true
        } else {
            false
        }
    }
    // calculates the distance between two points
    pub fn distance(p1: &Point, p2: &Point) -> f32 {
        let dx: f32 = p1.x - p2.x;
        let dy: f32 = p1.y - p2.y;
        ((dx * dx) + (dy * dy)).sqrt()

    }
}

struct Rectangle {
    p1: Point,
    p2: Point,
}

impl Rectangle {
    // new Rectangle with the given corner points
    pub fn new(px: Point, py: Point) -> Rectangle {
        Rectangle { p1: px, p2: py }
    }
    // calculates the area of this rectangle
    pub fn area(&self) -> f32 {
        if (self.p1.x - self.p2.x) * (self.p1.y + self.p2.y) < 0.0 {
            (self.p1.x - self.p2.x) * (self.p1.y + self.p2.y) * -1.0
        } else {
            (self.p1.x - self.p2.x) * (self.p1.y + self.p2.y)
        }
    }
    // Tests wether a point lies in this rectangle
    pub fn contains(&self, p: &Point) -> bool {
        if (((p.x < self.p1.x) && (p.x > self.p2.x)) || ((p.x > self.p1.x) && (p.x < self.p2.x))) &&
           (((p.y < self.p1.y) && (p.y > self.p2.y)) || ((p.y > self.p1.y) && (p.y < self.p2.y))) {
            true
        } else {
            false
        }
    }
}
