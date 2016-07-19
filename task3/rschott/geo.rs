fn main() {
    let p = Point::new(3.14, 4.56);
    let q = Point::origin();
    println!("{}", p.is_origin());
    println!("{}", q.is_origin());
    println!("Distance: {}", Point::distance(&p, &q));

    let s = Rectangle::new(p, q);
    println!("Area: {}", s.area());
    println!("Contains? {}", s.contains(Point::new(5.6, 1.4)));
    println!("Contains? {}", s.contains(Point::new(1.2, 3.0)));

}

struct Point {
    x: f32,
    y: f32,
}

struct Rectangle {
    a: Point,
    b: Point,
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
    pub fn distance(p: &Point, q: &Point) -> f32 {
        ((q.x - p.x) * (q.x - p.x) + (q.y - p.y) * (q.y - p.y)).sqrt()
    }
}
impl Rectangle {
    pub fn new(x: Point, y: Point) -> Rectangle {
        Rectangle { a: x, b: y }
    }
    pub fn area(&self) -> f32 {
        ((self.a.x - self.b.x) * (self.a.y - self.b.y)).abs()
    }
    pub fn contains(&self, p: Point) -> bool {
        if self.a.x > self.b.x {
            if self.a.y > self.b.y {
                if self.b.x <= p.x && p.x <= self.a.x && self.b.y <= p.y && p.y <= self.a.y {
                    return true;
                } else {
                    return false;
                }
            } else {
                if self.b.x <= p.x && p.x <= self.a.x && self.a.y <= p.y && p.y <= self.b.y {
                    return true;
                } else {
                    return false;
                }
            }
        } else {
            if self.a.y > self.b.y {
                if self.a.x <= p.x && p.x <= self.b.x && self.b.y <= p.y && p.y <= self.a.y {
                    return true;
                } else {
                    return false;
                }
            } else {
                if self.a.x <= p.x && p.x <= self.b.x && self.a.y <= p.y && p.y <= self.b.y {
                    return true;
                } else {
                    return false;
                }
            }
        }
















        if self.b.x <= p.x && p.x <= self.a.x && self.b.y <= p.y && p.y <= self.a.y {
            true
        } else {
            false
        }
    }
}
