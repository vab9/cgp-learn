#[derive(Debug, Copy, Clone)]
struct Point {
    x: f32,
    y: f32,
}

impl Point {
    fn new(x: f32, y: f32) -> Self {
        Point { x: x, y: y }
    }

    fn origin() -> Self {
        Point { x: 0.0, y: 0.0 }
    }

    fn is_origin(&self) -> bool {
        if self.x == 0.0 && self.y == 0.0 {
            return true;
        } else {
            return false;
        }
    }

    fn distance(p1: Point, p2: Point) -> f32 {
        ((p1.x - p2.x).powi(2) + (p1.y - p2.y).powi(2)).sqrt()
    }
}

#[derive(Debug, Copy, Clone)]
struct Rectangle {
    a: Point,
    b: Point,
}

impl Rectangle {
    fn new(a: Point, b: Point) -> Self {
        Rectangle { a: a, b: b }
    }

    fn area(&self) -> f32 {
        (self.b.x - self.a.x) * (self.b.y - self.a.y)
    }

    fn contains(&self, p: Point) -> bool {
        if p.x > self.a.x && p.x < self.b.x && p.y > self.a.y && p.y < self.b.y {
            return true;
        } else {
            return false;
        }
    }
}

fn main() {
    let point1 = Point { x: 3.0, y: 5.0 };
    let point2 = Point::new(20.0, 10.0);
    let point3 = Point { x: 7.0, y: 7.0 };
    let rect1 = Rectangle::new(point1, point2);
    let point4 = Point::origin();
    println!("{}", point4.is_origin());
    println!("{}", rect1.area());
    println!("{}", Point::distance(point1, point2));
    println!("{}", rect1.contains(point3));

}
