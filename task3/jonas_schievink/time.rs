fn main() {
    for hour in 0..24 {
        println!("{} -> {}", hour, greeting(hour));
    }
}

fn greeting(hour: u8) -> String {
    match hour {
        8 ... 12 => "Guten Morgen".to_string(),
        18 ... 22 => "Guten Abend".to_string(),
        0 ... 5 => format!("Warum bist du denn um {} Uhr noch wach?", hour),
        _ => "Hallo".to_string(),
    }
}
