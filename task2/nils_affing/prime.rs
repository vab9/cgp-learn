fn main() {
    let mut hits = 0;
    let mut current = 1;

    while hits <= 20 {
        if is_prime(current) {
            println!("Primzahl: {}. {}", hits, current);
            hits += 1;
        }
        current += 1;
    }
}

fn is_prime(n: u32) -> bool {
    for current in 2..n {
        if n % current == 0 {
            return false;
        }
    }
    true
}
