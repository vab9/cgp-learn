fn main() {
    let p1 = Point::origin();
    let p2 = Point::new(1.0, 1.0);
    let p3 = Point::new(2.0, 2.0);
    let p4 = Point::new(0.5, 0.5);

    println!("{}", p1.is_origin());
    println!("{}", p2.is_origin());
    println!("{}", Point::distance(&p1, &p2));

    let rect = Rectangle::new(p1, p2);

    println!("{}", rect.area());
    println!("{}", rect.contains(&p3));
    println!("{}", rect.contains(&p4));
}

struct Point {
    x: f32,
    y: f32,
}

impl Point {
    pub fn new(in_x: f32, in_y: f32) -> Point {
        Point { x: in_x, y: in_y }
    }

    pub fn origin() -> Point {
        Point { x: 0.0, y: 0.0 }
    }

    pub fn is_origin(&self) -> bool {
        self.x == 0.0 && self.y == 0.0
    }

    pub fn distance(p1: &Point, p2: &Point) -> f32 {
        ((p1.x - p2.x) * (p1.x - p2.x) + (p1.y - p2.y) * (p1.y - p2.y)).sqrt()
    }
}

struct Rectangle {
    p1: Point,
    p2: Point,
}

impl Rectangle {
    pub fn new(in_p1: Point, in_p2: Point) -> Rectangle {
        Rectangle {
            p1: in_p1,
            p2: in_p2,
        }
    }

    pub fn area(&self) -> f32 {
        ((self.p1.x - self.p2.x) * (self.p1.y - self.p2.y)).abs()
    }

    pub fn contains(&self, p: &Point) -> bool {
        let mut cx = false;
        let mut cy = false;

        if self.p1.x < self.p2.x && p.x >= self.p1.x && p.x <= self.p2.x {
            cx = true;
        } else if self.p1.x > self.p2.x && p.x <= self.p1.x && p.x >= self.p2.x {
            cx = true;
        }

        if self.p1.y < self.p2.y && p.y >= self.p1.y && p.y <= self.p2.y {
            cy = true;
        } else if self.p1.y > self.p2.y && p.y <= self.p1.y && p.y >= self.p2.y {
            cy = true;
        }

        cx && cy
    }
}
