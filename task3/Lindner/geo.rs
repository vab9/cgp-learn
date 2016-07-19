fn main() {
    let pt = Point::new(5.0,2.0);

    println!("is {},{} origin => {}",pt.x, pt.y, pt.is_origin());
    println!("distance between origin and pt => {}", pt.distance(Point::origin()));

}

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
            x: 0.0,
            y: 0.0,
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
        ((self.x - p.x).powi(2) + (self.y - p.y).powi(2)).sqrt()
    }
}
