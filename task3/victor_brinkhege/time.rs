fn time(t: u8) -> String {
    match t {
        0...5 => format!{"Warum bist du um {} noch wach?", t},
        8...12 => "Guten Morgen".to_string(),
        18...22 => "Guten Abend".to_string(),
        _ => "Hallo".to_string(),
    }

}

fn main() {
    println!("3 Uhr: {}", time(3));
}
