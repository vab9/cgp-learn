fn main() {

    println!("{}", get_time(4));
}
fn get_time(time: u8) -> String {
    match time {
        8...12 => "Guten Morgen".to_string(),
        18...22 => "Guten Abend".to_string(),
        0...5 => format!("Warum bist du um {} Uhr noch wach?", time),
        _ => "Hallo".to_string(),
    }
}
