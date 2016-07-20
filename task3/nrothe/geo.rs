fn main() {
    let p = Point::origin();
    println!("{}", &p.is_origin());
    let q = Point::new(5.0, 5.0);
    println!("{}", &q.is_origin());
    println!("{}", distance(&q, &p));

    let r = Rectangle::new(p, q);
    let s = Point::new(3.0, 3.0);

    println!("{}", r.area());
    println!("{}", r.contains(&s));
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
}

fn distance(q: &Point, p: &Point) -> f32 {
    (((q.x - p.x) * (q.x - p.x)) + ((q.y - p.y) * (q.y - p.y))).sqrt()
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

    pub fn contains(&self, p: &Point) -> bool {
        (((p.x <= self.p1.x && p.x >= self.p2.x) || (p.x >= self.p1.x && p.x <= self.p2.x)) &&
         ((p.y <= self.p1.y && p.y >= self.p2.y) || (p.y >= self.p1.y && p.y <= self.p2.y)))
    }
}
