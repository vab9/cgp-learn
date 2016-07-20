/// Print Greeting corresponding to time

fn main() {
    for i in 0..25 {
        println!("[{} Uhr] {}", i, greeting(i));
    }
}

fn greeting(time: u8) -> String {
    if time >= 8 && time <= 12 {
        "Guten Morgen".to_string()
    } else if time >= 18 && time <= 22 {
        "Guten Abend".to_string()
    } else if time <= 5 {
        format!("Warum bist du um {} Uhr noch wach?", time)
    } else {
        "Hallo".to_string()
    }
}
