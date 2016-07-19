fn main() {
    println!("{}", greet(5));
    println!("{}", greet(11));
    println!("{}", greet(19));
    println!("{}", greet(7));
}

fn greet(time: u8) -> String {
    match time {
        0...5 => format!("Warum bist du denn um {} Uhr noch wach", time),
        8...12 => "Guten Morgen".to_string(),
        18...22 => "Guten Abend".to_string(),
        _ => "Hallo".to_string(),
    }
}
