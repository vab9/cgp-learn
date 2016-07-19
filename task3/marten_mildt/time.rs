fn main() {
    println!("{}", greeting(10));
    println!("{}", greeting(14));
    println!("{}", greeting(20));
}

fn greeting(hour: u8) -> String {
    match hour {
        8...12 => "Guten Morgen".to_string(),
        18...22 => "Guten Abend".to_string(),
        _ => "Hallo".to_string(),
    }
}
