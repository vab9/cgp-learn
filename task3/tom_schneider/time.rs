fn main() {
    let mut a: u8 = 4;
    println!("{}", greet(&a));
    a = 10;
    println!("{}", greet(&a));
    a = 19;
    println!("{}", greet(&a));
    a = 13;
    println!("{}", greet(&a));
}

fn greet(time: &u8) -> String {
    if *time < 5 {

        format!("Warum bist du denn um {} Uhr noch wach?", *time)
    } else if *time >= 8 && *time < 12 {
        "Guten Morgen".to_string()
    } else if *time >= 18 && *time < 22 {
        "Guten Abend".to_string()
    } else {
        "Hallo".to_string()
    }
}
