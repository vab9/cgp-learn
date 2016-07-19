/// Displays greetings based on time
fn main() {
    for i in 0..23 {
        println!("{}", greetings(i));
    }
}

fn greetings(t: u8) -> String {
    match t {
        0...5 => format!("Warum bist du denn um {} Uhr noch wach?", t).to_string(),
        8...12 => "Guten Morgen.".to_string(),
        18...22 => "Guten Abend.".to_string(),
        _ => "Hallo".to_string(),
    }
}
