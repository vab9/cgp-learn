fn main() {
    println!("{}", time(9));
    println!("{}", time(15));
    println!("{}", time(19));
    println!("{}", time(4));
}

fn time(hour: u8) -> std::string::String {
    if hour >= 8 && hour <= 12 {
        "Guten Morgen".to_string()
    } else if hour >= 18 && hour <= 22 {
        "Guten Abend".to_string()
    } else if hour >= 0 && hour <= 5 {
        format!("Warum bist du denn um {} Uhr noch wach?", hour)
    } else {
        "Hallo".to_string()
    }

}
