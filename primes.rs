fn main() {
    let mut z = 1;
    let mut counter = 1;
    while z < 21 {
        if is_prime(counter) == true {
            println!("{}. Primzahl: {}", z, counter);
            z = z + 1;
        } else {}
        counter = counter + 1;
    }
}

fn is_prime(u: u32) -> bool {
    let mut prime = true;

    for i in 2..u {
        if u % i == 0 {
            prime = false;
        }
    }
    prime
}
