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
    pub fn distance(&self, sec: Point) -> f32 {
        (((sec.x - self.x) * (sec.x - self.x)) + ((sec.y - self.y) * (sec.y - self.y))).sqrt()
    }
}
fn main() {
    let eins = Point::origin();
    let zwei = Point::new(5.0, 0.0);
    println!("{}", eins.is_origin());
    println!("{}", zwei.is_origin());
    println!("{}", eins.distance(zwei));
}
