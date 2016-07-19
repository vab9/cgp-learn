fn main() {
    println!("{}", greetings(9));
    println!("{}", greetings(12));
    println!("{}", greetings(15));
    println!("{}", greetings(18));
    println!("{}", greetings(20));
}

fn greetings(t: u8) -> &'static str {
    if t >= 8 && t <= 12 {
        "Guten Morgen"
    } else if t >= 18 && t <= 22 {
        "Guten Abend"
    } else {
        "Hallo"
    }
}
