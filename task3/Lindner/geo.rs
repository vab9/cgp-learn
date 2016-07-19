fn main() {
    let pt = Point::new(5.0, 2.0);

    println!("is {},{} origin => {}", pt.x, pt.y, pt.is_origin());
    println!("distance between origin and pt => {}",
             pt.distance(Point::origin()));

    let rt = Rectangle::new();

    println!("rectangle defined by vector {},{} and {},{}",
             rt.x[0],
             rt.x[1],
             rt.y[0],
             rt.y[1]);
    println!("area of rectangle => {}", rt.area());

    println!("is Point {},{} in rectangle =>{}",
             pt.x,
             pt.y,
             rt.contains(&pt));

    let pti = Point::new(0.5, 0.5);

    println!("is Point {},{} in rectangle =>{}",
             pti.x,
             pti.y,
             rt.contains(&pti));
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
        Point { x: 0.0, y: 0.0 }
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

/// *start pretending rectangle being in a new file*
/// spans a rectangle from one location via 2 vectors
struct Rectangle {
    x: Vec<f64>,
    y: Vec<f64>,
}

impl Rectangle {
    fn new() -> Rectangle {
        Rectangle {
            x: vec![1.0, 0.0],
            y: vec![0.0, 1.0],
        }
    }

    fn area(&self) -> f64 {
        (self.x[0].powi(2) + self.x[1].powi(2)).sqrt() *
        (self.y[0].powi(2) + self.y[1].powi(2)).sqrt()
    }

    fn contains(&self, p: &Point) -> bool {
        let min_x = self.x[0].min(self.y[0]);
        let min_y = self.x[1].min(self.y[1]);
        let max_x = self.x[0].max(self.y[0]);
        let max_y = self.x[1].max(self.y[1]);

        if p.x < min_x || p.x > max_x || p.y < min_y || p.y > max_y {
            false
        } else {
            true
        }
    }
}
