fn main() {
    let p = Point::origin();
    println!("{}", &p.is_origin());
    let q = Point::new(5.0, 5.0);
    println!("{}", &q.is_origin());
    println!("{}", &q.distance(&p));

    let r = Rectangle::new(p, q);
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
        self.x == 0.0 && self.y == 0.0
    }

    pub fn distance(&self, p: &Point) -> f32 {
        (((self.x - p.x) * (self.x - p.x)) + ((self.y - p.y) * (self.y - p.y))).sqrt()
    }
}

struct Rectangle {
    p1: Point,
    p2: Point,
    length: f32,
    width: f32,
}

impl Rectangle {
    pub fn new(p_one: Point, p_two: Point) -> Rectangle {
        Rectangle {
            length: (&p_one.x - &p_two.x),
            width: (&p_one.y - &p_two.y),
            p1: p_one,
            p2: p_two,
        }
    }

    pub fn area(&self) -> f32 {
        (self.length * self.width).abs()
    }
}
