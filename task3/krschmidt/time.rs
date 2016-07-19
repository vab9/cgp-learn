fn main() {
    println!("{}", time_greet(10));
    println!("{}", time_greet(13));
    println!("{}", time_greet(19));
    println!("{}", time_greet(4));
}

fn time_greet(hour: u8) -> String {
    match hour {
        8...12 => "Guten Morgen".to_string(),
        18...22 => "Guten Abend".to_string(),
        0...5 => format!("Warum bist du denn um {} Uhr noch wach?", hour),
        _ => "Hallo".to_string(),
    }

}
