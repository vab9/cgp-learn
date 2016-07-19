#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

impl Point {
    pub fn new() -> Point {
        Point::origin()
    }
    pub fn origin() -> Point {
        Point { x: 0.0, y: 0.0 }
    }
    pub fn is_origin(&self) -> bool {
        self.x == 0.0 && self.y == 0.0
    }
    pub fn distance(&self, p: &Point) -> f32 {
        let distx = self.x - p.x;
        let disty = self.y - p.y;
        (distx * distx + disty * disty).sqrt()
    }
}

fn main() {

    let p1 = Point { x: 10.0, y: 10.0 };
    let p2 = Point::origin();
    println!("Ist der Punkt im Ursprung? {}", p2.is_origin());
    println!("Die Distance zwischen {:?} und {:?} ist {}",
             p1,
             p2,
             p1.distance(&p2));


}
