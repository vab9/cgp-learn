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
    pub fn distance(&self, sec: &Point) -> f32 {
        (((sec.x - self.x) * (sec.x - self.x)) + ((sec.y - self.y) * (sec.y - self.y))).sqrt()
    }
}
struct Rectangle {
    start: Point,
    heig: f32,
    leng: f32,
}
impl Rectangle {
    pub fn new(p: Point, h: f32, l: f32) -> Rectangle {
        Rectangle {
            start: p,
            heig: h,
            leng: l,
        }
    }
    pub fn area(&self) -> f32 {
        self.heig * self.leng
    }
    pub fn contains(&self, p: &Point) -> bool {
        (((self.start.x <= p.x && (self.start.x + self.leng) >= p.x) ||
          (self.start.x >= p.x && (self.start.x + self.leng) <= p.x)) &&
         ((self.start.y <= p.y) && (self.start.y + self.heig >= p.y) ||
          (self.start.y <= p.y) && (self.start.y + self.heig >= p.y)))
    }
}
fn main() {
    let eins = Point::origin();
    let zwei = Point::new(5.0, 0.0);
    println!("{}", &eins.is_origin());
    println!("{}", &zwei.is_origin());
    println!("{}", &eins.distance(&zwei));
    println!("");
    let rect = Rectangle::new(zwei, 5.0, 5.0);
    println!("{}", &rect.area());
}
