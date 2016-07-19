fn main() {
    let a = Point::new(3.4, 7.8);
    println!("A: x:{} y:{}  origin?: {}", a.x, a.y, a.is_origin());
    let b = Point::origin();
    println!("B: x:{} y:{}  origin?: {}", b.x, b.y, b.is_origin());
    println!("Distance A-B {}", Point::distance(&a, &b));
    let c = Point::new(2.75, 6.0);
    println!("C: x:{} y:{}  origin?: {}", c.x, c.y, c.is_origin());
    let d = Rectangle::new(a, b);
    println!("Rectangle A-B Area:{}  Contains C?: {}",
             d.area(),
             d.contains(&c));
}

struct Point {
    x: f32,
    y: f32,
}

impl Point {
    pub fn origin() -> Point {
        Point { x: 0.0, y: 0.0 }
    }

    pub fn new(x: f32, y: f32) -> Point {
        Point { x: x, y: y }
    }

    pub fn is_origin(&self) -> bool {
        ((self.x == 0.0) && (self.y == 0.0))
    }

    pub fn distance(a: &Point, b: &Point) -> f32 {
        (((a.x - b.x) * (a.x - b.x)) + ((a.y - b.y) * (a.y - b.y))).sqrt()
    }
}

struct Rectangle {
    a: Point,
    b: Point,
}

impl Rectangle {
    pub fn new(p: Point, q: Point) -> Rectangle {
        Rectangle { a: p, b: q }
    }

    pub fn area(&self) -> f32 {
        ((self.a.x - self.b.x) * (self.a.y - self.b.y) * (self.a.x - self.b.x) *
         (self.a.y - self.b.y))
            .sqrt()
        // quadrieren und wurzelziehen um positives ergebniss zu garantieren
    }

    pub fn contains(&self, p: &Point) -> bool {
        if p.x >= self.a.x && p.x <= self.b.x || p.x >= self.b.x && p.x <= self.a.x {
            if p.y >= self.a.y && p.x <= self.b.y || p.y >= self.b.y && p.y <= self.a.y {
                true
            } else {
                false
            }
        } else {
            false
        }
    }
}
