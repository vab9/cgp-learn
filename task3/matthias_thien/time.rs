fn main() {
    let time: [u8; 4] = [3, 7, 9, 19];
    println!("{}", message(time[0]));
    println!("{}", message(time[1]));
    println!("{}", message(time[2]));
    println!("{}", message(time[3]));
}
fn message(input: u8) -> String {
    match input {
        0...5 => format!("Warum bist du um {} Uhr noch wach?", input),
        8...12 => "Guten Morgen".to_string(),
        18...22 => "Guten Abend".to_string(),
        _ => "Hallo".to_string(),
    }
}
