

fn main() {
    for n in 0..23 {
        println!("It\'s {} o\'clock, {}", n, clock(n));
    }
}

fn clock(n: u8) -> String {
    match n {
        8...12 => "Guten Morgen".to_string(),
        18...22 => "Guten Abend".to_string(),
        0...5 => format!("Warum bist du denn um {} noch wach", n),
        _ => "Hallo".to_string(),
    }
}
