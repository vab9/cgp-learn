fn main() {
    let mut counter = 1;
    for z in 1.. {
        if is_prime(z) {
            println!("{}. Primzahl: {}", counter, z);
            counter += 1;
            if counter == 21 {
                break;
            }
        }
    }
}

fn is_prime(u: u32) -> bool {
    let prime = true;

    for i in 2..u {
        if u % i == 0 {
            return false;
        }
    }
    prime
}
