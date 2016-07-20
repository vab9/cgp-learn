fn main() {
    let p1_1 = Point::new(1.0, 1.0);
    let p0_0 = Point::origin();

    println!("1_1 is origin? {}", p1_1.is_origin());
    println!("0_0 is origin? {}", p0_0.is_origin());
    println!("dist(0_0, 1_1): {}", Point::distance(&p0_0, &p1_1));

    let rp0_0p1_1 = Rect::new(p0_0, p1_1);
    println!("area of rp0_0p1_1: {}", rp0_0p1_1.area());
    println!("rp0_0p1_1 contians 0.5_0.5? {}",
             rp0_0p1_1.contains(&Point::new(0.5, 0.5)));
}

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

    fn distance(p1: &Point, p2: &Point) -> f32 {
        let dx = p2.x - p1.x;
        let dy = p2.y - p1.y;
        (dx * dx + dy * dy).sqrt()
    }
}

struct Rect {
    up_left: Point,
    down_right: Point,
}

impl Rect {
    fn new(p1: Point, p2: Point) -> Rect {
        Rect {
            up_left: Point::new(if p1.x <= p2.x {
                                    p1.x
                                } else {
                                    p2.x
                                },
                                if p1.y >= p2.y {
                                    p1.y
                                } else {
                                    p2.y
                                }),
            down_right: Point::new(if p1.x > p2.x {
                                       p1.x
                                   } else {
                                       p2.x
                                   },
                                   if p1.y < p2.y {
                                       p1.y
                                   } else {
                                       p2.y
                                   }),
        }
    }

    fn area(&self) -> f32 {
        let tmp = Point::new(self.up_left.x, self.down_right.y);
        Point::distance(&tmp, &self.up_left) * Point::distance(&tmp, &self.down_right)
    }

    fn contains(&self, p: &Point) -> bool {
        p.x >= self.up_left.x && p.x <= self.down_right.x && p.y >= self.down_right.y &&
        p.y <= self.up_left.y
    }
}
