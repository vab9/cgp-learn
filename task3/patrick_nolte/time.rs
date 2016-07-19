fn main() {
    println!("{}", get_message_at_time(3u8));
}

fn get_message_at_time(time: u8) -> String {
    if time > 8 && time < 12 {
        String::from("Guten Morgen")
    } else if time > 18 && time < 22 {
        String::from("Guten Abend")
    } else if time > 0 && time < 5 {
        String::from(format!("Warum bist du denn um {} Uhr noch wach?", time))
    } else {
        String::from("Hallo")
    }
}
