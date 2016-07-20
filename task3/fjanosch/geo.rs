fn main() {
    let p = Point::new(3.0, 1.0);
    let o = Point::origin();
    let r = Rectangle::new(p, o);
    let x = r.area();
    println!("{:?}", x);

}

#[derive(Debug)]
struct Point {
    X: f32,
    Y: f32,
}

impl Point {
    pub fn origin() -> Point {
        Point { X: 0.0, Y: 0.0 }
    }

    pub fn new(a: f32, b: f32) -> Point {
        Point { X: a, Y: b }
    }

    pub fn is_origin(&self) -> bool {
        if self.X == 0.0 && self.Y == 0.0 {
            return true;
        }
        false
    }

    fn distance(a: Point, b: Point) {
        let x = a.X - b.X;
        let y = a.Y - b.Y;
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
        let x = A.x - B.x;
        let y = A.y - B.y;
        let mut area = x * y;
        if area < 0.0 {
            area = -area;
        }
        area
    }

    pub fn contains(&self, p: Point) -> bool {
        if A.x < B.x {
            if A.y < B.y {
                if p.y > A.y && p.y < B.y && p.x < A.x && p.x > B.x {
                    return true;
                }
            } else if A.y > B.y {
                if p.y < A.y && p.y > B.y && p.x < A.x && p.x > B.x {
                    return true;
                }
            }
            // } else if A.x > B.x {
        } else if A.x > B.x {

            if A.y < B.y {
                if p.y > A.y && p.y < B.y && p.x > A.x && p.x < B.x {
                    return true;
                }
            } else if A.y > B.y {
                if p.y < A.y && p.y > B.y && p.x > A.x && p.x < B.x {
                    return true;
                }
            }
        }
        false
    }
}
