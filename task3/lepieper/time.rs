fn main() {
    println!("{}", greetings(9));
    println!("{}", greetings(12));
    println!("{}", greetings(15));
    println!("{}", greetings(18));
    println!("{}", greetings(20));
    println!("{}", greetings(3));
}

fn greetings(t: u8) -> String {
    if t >= 8 && t <= 12 {
        "Guten Morgen".to_string()
    } else if t >= 18 && t <= 22 {
        "Guten Abend".to_string()
    } else if t <= 5 {
        format!("Warum bist du denn um {} Uhr noch wach?", t)
    } else {
        "Hallo".to_string()
    }
}
