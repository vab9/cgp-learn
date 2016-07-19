fn main() {
    println!("{}", greeting(3));
}

fn greeting(time: u8) -> String {
    if time <= 5 {
        format!("Warum bist du denn noch um {} Uhr wach", time)
    } else if time >= 8 && time <= 12 {
        String::from("Guten Morgen")
    } else if time >= 18 && time <= 22 {
        String::from("Guten Abend")
    } else {
        String::from("Hallo")
    }
}
