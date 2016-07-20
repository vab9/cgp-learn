
// TODO

fn foo<T: std::ops::Add + std::ops::Mul>
    (x: T,
     y: T)
     -> (<T as std::ops::Add>::Output, <T as std::ops::Mul>::Output) {
    ((x + y), (x * y))
}


fn main() {
    println!("{:?}", foo(3, 4));
}
