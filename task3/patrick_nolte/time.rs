fn main() {
    println!("{}", get_message_at_time(3));
}

fn get_message_at_time(time: u8) -> String {
    match time {
        8...12 => "Guten Morgen".to_string(),
        18...22 => "Guten Abend".to_string(),
        0...5 => format!("Warum bist du denn um {} Uhr noch wach?", time),
        _ => "Hallo".to_string(),
    }
}
