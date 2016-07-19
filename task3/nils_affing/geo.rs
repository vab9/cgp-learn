#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

impl Point {
    fn new(x: f32, y: f32) -> Point {
        Point { x: x, y: y }
    }

    fn origin() -> Point {
        Point { x: 0.0, y: 0.0 }
    }

    fn is_origin(&self) -> bool {
        self.x == 0.0 && self.y == 0.0
    }

    fn distance(&self, other: &Point) -> f32 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;

        (dx * dx + dy * dy).sqrt()
    }
}

#[derive(Debug)]
struct Rectangle {
    first: Point,
    second: Point,
}

impl Rectangle {
    fn new() -> Rectangle {
        Rectangle {
            first: Point::origin(),
            second: Point::origin(),
        }
    }

    fn area(&self) -> f32 {
        let dx = self.first.x - self.second.x;
        let dy = self.first.y - self.second.y;

        dx * dy
    }

    fn contains(&self, p: &Point) -> bool {
        let mut minX = 0.0;
        let mut maxX = 0.0;

        if self.first.x < self.second.x {
            minX = self.first.x;
            maxX = self.second.x;
        } else {
            minX = self.second.x;
            maxX = self.first.x;
        }

        let mut minY = 0.0;
        let mut maxY = 0.0;

        if self.first.y < self.second.y {
            minY = self.first.y;
            maxY = self.second.y;
        } else {
            minY = self.second.y;
            maxY = self.first.y;
        }

        minX <= p.x && minY <= p.y && maxX >= p.x && maxY >= p.y
    }
}

fn main() {
    let mut a = Point::origin();
    let mut b = Point::new(100.0, 100.0);

    println!("Distanz: {}", a.distance(&b));

    let mut rec = Rectangle::new();
    rec.first = a;
    rec.second = b;

    let mut test = Point::new(50.0, 50.0);

    println!("{:#?}", rec);
    println!("{:#?} liegt in dem Rectangle {:#?} -> {}",
             test,
             rec,
             rec.contains(&test));
}
