struct Point {
    x: f32,
    y: f32,
}

impl Point {
    pub fn new(a: f32, b: f32) -> Point {

        Point { x: a, y: b }
    }

    pub fn origin() -> Point {

        Point::new(0.0, 0.0)
    }

    pub fn is_origin(&self) -> bool {

        if self.x == 0.0 && self.y == 0.0 {
            true
        } else {
            false
        }
    }

    pub fn distance(p1: Point, p2: Point) -> f32 {

        let x = p1.x - p2.x;
        let y = p1.y - p2.y;

        (x * x + y * y).sqrt()
    }
}

struct Rectangle {
    top_left: Point,
    width: f32,
    height: f32,
}

impl Rectangle {
    pub fn new(p: Point, w: f32, h: f32) -> Rectangle {

        Rectangle {
            top_left: p,
            width: w,
            height: h,
        }

    }

    pub fn area(&self) -> f32 {

        self.width * self.height

    }

    pub fn contains(&self, point: Point) -> bool {

        if point.x < self.top_left.x || point.x > self.top_left.x + self.width ||
           point.y > self.top_left.y || point.y < self.top_left.y + self.height {

            false
        } else {
            true
        }
    }
}

fn main() {

    let p1 = Point::new(3.0, 5.0);
    let p2 = Point::origin();

    println!("Punkt 1, is_origin : {}", p1.is_origin());
    println!("Punkt 2, is_origin : {}", p2.is_origin());
    println!("Distanz: {}", Point::distance(p1, p2));

    let r: Rectangle = Rectangle::new(Point::new(4.0, 4.0), 2.0, 1.0);
    let p3 = Point::new(2.0, 2.0);

    println!("Rechteck Flaeche: {}", r.area());
    println!("Punkt1 im Rechteck: {}", r.contains(p3));

}
