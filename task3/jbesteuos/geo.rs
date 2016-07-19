fn main() {}

struct Point {
    x: f32,
    y: f32,
}

impl Point {
    pub fn new(x: f32, y: f32) -> Point {
        Point { x: x, y: y }
    }

    pub fn origin() -> Point {
        Point { x: 0.0, y: 0.0 }
    }

    pub fn is_origin(&self) -> bool {
        if self.x == 0.0 && self.y == 0.0 {
            return true;
        } else {
            false
        }
    }

    pub fn distance(m: Point, n: Point) -> f32 {

        let dx = m.x - n.x;
        let dy = m.y - n.y;
        ((dx * dx) + (dy * dy)).sqrt()
    }
}


struct Rectangle {
    topleft: Point,
    len: f32,
    high: f32,
}

impl Rectangle {
    pub fn new(p: Point, len: f32, high: f32) -> Rectangle {
        Rectangle {
            topleft: p,
            len: len,
            high: high,
        }
    }

    pub fn area(&self) -> f32 {
        self.len * self.high
    }

    pub fn contains(&self, point: Point) -> bool {

        if self.topleft.x <= point.x && self.topleft.y <= point.y &&
           self.len + self.topleft.x >= point.x &&
           self.high + self.topleft.y >= point.y {
            return true;
        } else {
            false
        }

    }
}
