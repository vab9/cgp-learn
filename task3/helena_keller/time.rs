fn greet(time: u8) -> String {

    match time {
        0...5 => format!("Warum bist du um {} Uhr noch wach?", time),
        6...7 | 13...17 | 22...24 => format!("Hallo"),
        8...12 => format!("Guten Morgen"),
        18...22 => format!("Guten Abend"),
        _ => format!("So eine Uhrzeit haben wir nicht."),
    }
}

fn main() {

    println!("3.00 Uhr: {}", greet(3));
    println!("Um 11.00 Uhr sagen wir {}.", greet(11));
    println!("Um 23.00 Uhr sagen wir {}.", greet(23));
    println!("Um 19.00 Uhr sagen wir {}.", greet(19));
    println!("Bei 30: {}", greet(30));

}
