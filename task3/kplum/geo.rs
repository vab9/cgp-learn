#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

impl Point {
    pub fn new(x: f32, y: f32) -> Point {
        Point { x: x, y: y }
    }

    pub fn origin() -> Point {
        Point::new(0.0, 0.0)
    }

    pub fn is_origin(&self) -> bool {
        self.x == 0.0 && self.y == 0.0
    }

    pub fn distance(p1: &Point, p2: &Point) -> f32 {
        let x_d = p1.x - p2.x;
        let y_d = p1.y - p2.y;
        (x_d * x_d + y_d * y_d).sqrt()
    }
}

#[derive(Debug)]
struct Rectangle {
    p1: Point,
    p2: Point,
}

impl Rectangle {
    pub fn new(p1: &Point, p2: &Point) -> Rectangle {
        Rectangle {
            p1: Point::new(p1.x, p1.y),
            p2: Point::new(p2.x, p2.y),
        }
    }

    pub fn area(&self) -> f32 {
        let x_d = (self.p1.x - self.p2.x).abs();
        let y_d = (self.p1.y - self.p2.y).abs();
        x_d * y_d
    }

    pub fn contains(&self, p: &Point) -> bool {

        let result = if self.p1.x < self.p2.x {
            if self.p1.y < self.p2.y {
                p.x >= self.p1.x && p.x <= self.p2.x && p.y >= self.p1.y && p.y <= self.p2.y
            } else {
                p.x >= self.p1.x && p.x <= self.p2.x && p.y <= self.p1.y && p.y >= self.p2.y
            }
        } else if self.p1.y < self.p2.y {
            p.x <= self.p1.x && p.x >= self.p2.x && p.y >= self.p1.y && p.y <= self.p2.y
        } else {
            p.x <= self.p1.x && p.x >= self.p2.x && p.y <= self.p1.y && p.y >= self.p2.y
        };
        result
    }
}

fn main() {
    let point = Point::new(1.0, 2.0);
    println!("{:?}", point);
    let point2 = Point::new(3.0, 4.0);
    let point3 = Point::new(100.0, 110.0);
    let point4 = Point::new(2.0, 3.0);
    let rect = Rectangle::new(&point, &point2);
    println!("area: {}", rect.area());
    println!("contains false {}", rect.contains(&point3));
    println!("contains true {}", rect.contains(&point));
    println!("contains true {}", rect.contains(&point4));
    println!("distance {}", Point::distance(&point2, &point3));
    println!("is origin true {}", Point::origin().is_origin());
    println!("is origin false {}", point.is_origin());

}
