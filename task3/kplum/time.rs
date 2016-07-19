fn main() {
    println!("{}", greet(11));
    println!("{}", greet(18));
    println!("{}", greet(1));
    println!("{}", greet(6));
}

fn greet(time: u8) -> String {
    if time <= 12 && time >= 8 {
        "Guten Morgen".to_string()
    } else if time >= 18 && time <= 22 {
        "Guten Abend".to_string()
    } else if time <= 5 {
        format!("Warum bist du denn um {} Uhr noch wach?!", time)
    } else {
        "Hallo!".to_string()
    }
}
