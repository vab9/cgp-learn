fn main() {
    for i in 0..24 {
        println!("um {} Uhr -> {}", i, greet_me(i));
    }
}

fn greet_me(time: u8) -> String {
    if time <= 24 {
        if time <= 12 && time >= 8 {
            "Guten Morgen".to_string()
        } else if time <= 22 && time >= 18 {
            "Guten Abend".to_string()
        } else if time <= 5 {
            format!("Warum bist du denn um {} Uhr noch wach?", time)
        } else {
            "Hallo".to_string()
        }
    } else {
        "invalid Time".to_string()
    }
}
