use std::borrow::Cow;
use std::str;
fn main() {
    let a = 6;
    let b = 20;
    let c = 3;
    println!("awake at {} => {}", a, get_gretting(a));
    println!("awake at {} => {}", b, get_gretting(b));
    println!("awake at {} => {}", c, get_gretting(c));
}

fn get_gretting(hour :u8) -> Cow<'static, str>{
    match hour {
        0...5 => Cow::Owned(format!("why are you still awake at {} am", hour)),
        8...12 => Cow::Borrowed("Good morning"),
        18...22 => Cow::Borrowed("Good afternoon"),
        _ => Cow::Borrowed("Hello"),
    }
}