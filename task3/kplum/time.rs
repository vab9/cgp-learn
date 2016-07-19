fn main() {
    println!("{}", greet(11));
    println!("{}", greet(18));
    println!("{}", greet(1));
    println!("{}", greet(6));
}

fn greet(time: u8) -> String {
    match time {
        8...12 => "Guten Morgen".to_string(),
        18...22 => "Guten Abend".to_string(),
        0...5 => format!("Warum bist du denn um {} Uhr noch wach?!", time),
        _ => "Hallo!".to_string(),
    }
}
