fn main() {
    for i in 0..24 {
        println!("um {} Uhr -> {}", i, greet_me(i));
    }
}

fn greet_me(time: u8) -> String {
    match time {
        8...12 => "Guten Morgen".to_string(),
        18...22 => "Guten Abend".to_string(),
        0...5 => format!("Warum bist du denn um {} Uhr noch wach?", time),
        _ => "Hallo".to_string(),
    }
}
