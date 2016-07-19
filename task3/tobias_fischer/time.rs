/// Displays greetings based on time
fn main() {
    for i in 0..23 {
        println!("{}", greetings(i));
    }
}

fn greetings(t: u8) -> String {
    if t <= 5 {
        format!("Warum bist du denn um {} Uhr noch wach?", t).to_string()
    } else if t >= 8 && t <= 12 {
        "Guten Morgen.".to_string()
    } else if t >= 18 && t <= 22 {
        "Guten Abend.".to_string()
    } else {
        "Hallo".to_string()
    }
}
