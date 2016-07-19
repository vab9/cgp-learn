

fn main() {
    let mut n: u8 = 0;
    while n < 24 {
        println!("It\'s {} o\'clock, {}", n, &clock(n));
        n += 1;
    }
}

fn clock(n: u8) -> String {
    if n >= 8 && n <= 12 {
        "Guten Morgen".to_string()
    } else if n >= 18 && n <= 22 {
        "Guten Abend".to_string()
    } else {
        "Hallo".to_string()
    }

}
