/// Do some testing with a Point and a Rectangle.
fn main() {
    println!("Creating Point 0,0");
    let p1 = Point::origin();
    println!("That lies in origin: {}", p1.is_origin());
    println!("Creating Point 1,1");
    let p2 = Point::new(1.0, 1.0);
    println!("That lies in origin: {}", p2.is_origin());
    println!("Distance between Points: {}", Point::distance(&p1, &p2));
    println!("");

    println!("Create Rectangle from the two Points");
    let rect = Rectangle::new(p1, p2);
    println!("Area of that thing is {}", rect.area());

    println!("Point 2,2 is in Rectangle: {}",
             rect.contains(Point::new(2.0, 2.0)));
    println!("Point 0.5,0.5 is in Rectangle: {}",
             rect.contains(Point::new(0.5, 0.5)));

}









struct Point {
    x: f32,
    y: f32,
}



impl Point {
    pub fn new(xv: f32, yv: f32) -> Point {
        Point { x: xv, y: yv }
    }

    pub fn origin() -> Point {
        Point { x: 0.0, y: 0.0 }
    }

    pub fn is_origin(&self) -> bool {
        if self.x == 0.0 && self.y == 0.0 {
            true
        } else {
            false
        }
    }

    pub fn distance(p1: &Point, p2: &Point) -> f32 {
        ((p1.x - p2.x) * (p1.x - p2.x) + (p1.y - p2.y) * (p1.y - p2.y)).sqrt()
    }
}





struct Rectangle {
    ul: Point, // upper left point
    br: Point, // lower right point
}

impl Rectangle {
    pub fn new(p1: Point, p2: Point) -> Rectangle {
        Rectangle { ul: p1, br: p2 }
    }

    pub fn area(&self) -> f32 {
        ((&self.ul.x - &self.br.x) * (&self.ul.y - &self.br.y)).abs()
    }


    pub fn contains(&self, p: Point) -> bool {
        // make sure that we compare the right values, so know which point's x value is greater
        let lower_x: f32;
        let upper_x = if self.ul.x > self.br.x {
            lower_x = self.br.x;
            self.ul.x
        } else {
            lower_x = self.ul.x;
            self.br.x
        };

        // same for y
        let lower_y: f32;
        let upper_y = if self.ul.y > self.br.y {
            lower_y = self.br.y;
            self.ul.y
        } else {
            lower_y = self.ul.y;
            self.br.y
        };

        // finally: compare
        if p.x > lower_x && p.x < upper_x && p.y > lower_y && p.y < upper_y {
            true
        } else {
            false
        }
    }
}
