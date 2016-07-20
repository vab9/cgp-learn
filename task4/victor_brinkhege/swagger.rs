use std::fmt::{Display, Formatter};

struct Swagger<T: Display> {
    x: T,
}

impl<T: Display> Display for Swagger<T> {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "#swag - {} - #yolo", self.x)
    }
}


fn main() {

    let s = Swagger { x: 35 };
    println!("{}", s);
}
