#[derive(Debug)]
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
        self.x == 0.0 && self.y == 0.0
    }
    pub fn distance(p1: &Point, p2: &Point) -> f32 {
        ((p1.x - p2.x) * (p1.x - p2.x) + (p1.y - p2.y) * (p1.y - p2.y)).sqrt()
    }
}

struct Rectangle {
    ul: Point,
    or: Point,
}
impl Rectangle {
    pub fn new(x: Point, y: Point) -> Rectangle {
        Rectangle { ul: x, or: y }
    }
    pub fn area(&self) -> f32 {
        let x = self.or.x;
        let y = self.ul.y;
        let t = Point::new(x, y);
        Point::distance(&self.ul, &t) * Point::distance(&self.or, &t)
    }
    pub fn contains(&self, p: &Point) -> bool {
        p.x >= self.ul.x && p.y >= self.ul.y && p.x <= self.or.x && p.y <= self.or.y
    }
}

fn main() {
    let a = Point::new(10.0, 10.0);
    let b = Point::origin();
    println!("{}", b.is_origin());
    println!("{}", Point::distance(&a, &b));
    let c = Rectangle::new(Point::origin(), Point::new(10.0, 10.0));
    println!("{}", c.area());
    println!("{}", c.contains(&Point::new(5.0, 5.0)));

    println!("{}", c.contains(&Point::new(11.0, 11.0)));
}
