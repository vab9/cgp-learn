fn main() {
    let a: u8 = 4;
    println!("{}", get_time(a));
}
fn get_time(time: u8) -> String {
    let s: String = time.to_string();
    let mut b: String = "Warum bist du um ".to_string();
    let c: String = " Uhr noch wach".to_string();
    b.push_str(&s);
    b.push_str(&c);
    match time {
        8...12 => "Guten Morgen".to_string(),
        18...22 => "Guten Abend".to_string(),
        0...5 => b,
        _ => "Hallo".to_string(),
    }
}
