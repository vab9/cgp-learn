fn main() {
    let mut n = 1;
    let mut x = 1;
    while n <= 20 {
        if is_prime(x) {
            println!("{}. Primzahl: {}", n, x);
            n += 1;
        }
        x += 1;
    }
}

fn is_prime(x: u32) -> bool {
    let mut prime = true;
    if x <= 1 {
        prime = false
    } else {
        for teiler in 2..(x / 2) + 1 {
            if x % teiler == 0 {
                prime = false;
                break;
            }
        }
    }
    prime
}
