fn main() {
    println!("{}", greetings(9));
    println!("{}", greetings(12));
    println!("{}", greetings(15));
    println!("{}", greetings(18));
    println!("{}", greetings(20));
    println!("{}", greetings(3));
}
fn greetings(t: u8) -> String {
    match t {
        8...12 => "Guten Morgen".to_string(),
        18...22 => "Guten Abend".to_string(),
        0...5 => format!("Warum bist du denn um {} Uhr noch wach?", t),
        _ => "Hallo".to_string(),
    }
}
