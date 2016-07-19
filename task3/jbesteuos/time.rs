fn main() {

    println!("{:?}", time(9));
    println!("{:?}", time(4));
}

fn time(n: u8) -> String {

    match n {
        8...12 => "Guten Morgen".to_string(),
        18...22 => "Guten Abend".to_string(),
        0...5 => format!("Warum bist du denn um {} Uhr noch wach?", n),
        _ => "Hallo".to_string(),
    }
}
