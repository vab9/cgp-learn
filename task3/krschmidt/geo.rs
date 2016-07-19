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
        self.x == 0.0 && self.y == 0.0
    }
    pub fn distance(p1: &Point, p2: &Point) -> f32 {
        ((p2.x - p1.x) * (p2.x - p1.x) + (p2.y - p1.y) * (p2.y - p1.y)).sqrt()
    }
}

struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

impl Rectangle {
    pub fn new(top_left: Point, bottom_right: Point) -> Rectangle {
        Rectangle {
            top_left: top_left,
            bottom_right: bottom_right,
        }
    }
    pub fn area(&self) -> f32 {
        (self.bottom_right.x - self.top_left.x) * (self.top_left.y - self.bottom_right.y).abs()
    }
    pub fn contains(&self, point: &Point) -> bool {
        self.bottom_right.x >= point.x && point.x > self.top_left.x &&
        self.bottom_right.y < point.y && point.y <= self.top_left.y

    }
}

fn main() {
    let p1: Point = Point::new(2.0, 5.0);
    let p2: Point = Point::new(4.0, 3.0);
    let p3: Point = Point::origin();
    let p4: Point = Point::new(3.0, 4.0);
    let dist = Point::distance(&p1, &p2);

    println!("p1: x: {}, y: {}", p1.x, p1.y);
    println!("p2: x: {}, y: {}", p2.x, p2.y);
    println!("p3: x: {}, y: {}", p3.x, p3.y);
    println!("p4: x: {}, y: {}", p4.x, p4.y);
    println!("distance: {}", dist);
    println!("p2 is origin: {}", p2.is_origin());
    println!("p3 is origin: {}", p3.is_origin());

    let rect: Rectangle = Rectangle::new(p1, p2);

    println!("tl: ({}, {}), br: ({}, {})",
             rect.top_left.x,
             rect.top_left.y,
             rect.bottom_right.x,
             rect.bottom_right.y);
    println!("area: {}", rect.area());
    println!("p3 in rectangle? {}", rect.contains(&p3));
    println!("p4 in rectangle? {}", rect.contains(&p4));
}
