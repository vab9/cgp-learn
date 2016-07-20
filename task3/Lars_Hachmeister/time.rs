fn main() {
    println!("{}", time(9));
    println!("{}", time(15));
    println!("{}", time(19));
    println!("{}", time(4));
}

fn time(hour: u8) -> String {
    match hour {
        8...12 => "Guten Morgen".to_string(),
        18...22 => "Guten Abend".to_string(),
        0...5 => format!("Warum bist du denn um {} Uhr noch wach?", hour),
        _ => "Hallo".to_string(),
    }

}
