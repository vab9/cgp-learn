use std::ops::{Add, Mul};

fn foo<T: Add + Mul + Copy>(x: T, y: T) -> (<T as Add>::Output, <T as Mul>::Output) {
    (x + y, x * y)
}


fn main() {
    println!("{:?}", foo(3, 4));
}
