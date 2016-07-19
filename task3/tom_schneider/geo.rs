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
        (self.x == 0.0 && self.y == 0.0)
    }
    pub fn distance(&self, p: &Point) -> f32 {
        ((self.x - p.x) * (self.x - p.x) + (self.y - p.y) * (self.y - p.y)).sqrt()
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
        self.ul.distance(&t).abs() * self.or.distance(&t).abs()
    }
    pub fn contains(&self, p: &Point) -> bool {
        (p.x >= self.ul.x && p.y >= self.ul.y && p.x <= self.or.x && p.y <= self.or.y)
    }
}

fn main() {
    let a = Point::new(10.0, 10.0);
    let b = Point::origin();
    println!("{}", b.is_origin());
    println!("{}", a.distance(&b));
    let c = Rectangle::new(Point::origin(), Point::new(10.0, 10.0));
    println!("{}", c.area());
    println!("{}", c.contains(&Point::new(5.0, 5.0)));

    println!("{}", c.contains(&Point::new(11.0, 11.0)));
}
