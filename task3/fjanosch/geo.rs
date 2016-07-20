fn main() {
    let p = Point::new(3.0, 1.0);
    let o = Point::origin();
    let r = Rectangle::new(p, o);
    let x = r.area();
    println!("{:?}", x);
    let c = Point::new(2.0, 0.5);
    let d = Point::new(0.0, -2.0);
    println!("{}", r.contains(c));
    println!("{}", r.contains(d));

}

#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

impl Point {
    pub fn origin() -> Point {
        Point { x: 0.0, y: 0.0 }
    }

    pub fn new(a: f32, b: f32) -> Point {
        Point { x: a, y: b }
    }

    pub fn is_origin(&self) -> bool {
        if self.x == 0.0 && self.y == 0.0 {
            return true;
        }
        false
    }

    fn distance(a: Point, b: Point) -> f32 {
        let x = a.x - b.x;
        let y = a.y - b.y;
        (x * x + y * y).sqrt()
    }
}

#[derive(Debug)]
struct Rectangle {
    A: Point,
    B: Point,
}

impl Rectangle {
    pub fn new(a: Point, b: Point) -> Rectangle {
        Rectangle { A: a, B: b }
    }

    pub fn area(&self) -> f32 {
        let x = self.A.x - self.B.x;
        let y = self.A.y - self.B.y;
        let mut area = x * y;
        if area < 0.0 {
            area = -area;
        }
        area
    }

    // min und max aus std würde das verbessern
    pub fn contains(&self, p: Point) -> bool {
        if self.A.x < self.B.x {
            if self.A.y < self.B.y {
                if p.y > self.A.y && p.y < self.B.y && p.x > self.A.x && p.x < self.B.x {
                    return true;
                }
            } else if self.A.y > self.B.y {
                if p.y < self.A.y && p.y > self.B.y && p.x > self.A.x && p.x < self.B.x {
                    return true;
                }
            }
        } else if self.A.x > self.B.x {

            if self.A.y < self.B.y {
                if p.y > self.A.y && p.y < self.B.y && p.x < self.A.x && p.x > self.B.x {
                    return true;
                }
            } else if self.A.y > self.B.y {
                if p.y < self.A.y && p.y > self.B.y && p.x < self.A.x && p.x > self.B.x {
                    return true;
                }
            }
        }
        false

    }
}
