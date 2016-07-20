fn main() {
    println!("{}", greeting(3));
}

fn greeting(time: u8) -> String {
    match time {
        0...5 => format!("Warum bist du denn noch um {} Uhr wach", time),
        8...12 => String::from("Guten Morgen"),
        18...22 => String::from("Guten Abend"),
        _ => String::from("Hallo"),
    }
}
