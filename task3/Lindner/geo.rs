fn main() {}

struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn new(x: f64, y: f64) -> Point {
        Point { x: x, y: y }
    }

    fn origin() -> Point {
        Point {
            x: 0.0_f64,
            y: 0.0_f64,
        }
    }

    fn is_origin(&self) -> bool {
        let p = Point::origin();
        if self.x == p.x && self.y == p.y {
            true
        } else {
            false
        }
    }

    fn distance(&self, p: Point) -> f64 {
        let a = self.x - p.x;
        let b = self.y - p.y;
        a.powi(2);
        b.powi(2);
        (a + b).sqrt()
    }
}