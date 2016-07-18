fn main() {
    let mut hits = 0;
    let mut current = 1;

    while hits <= 20 {
        hits = if is_prime(current) {
            println!("Primzahl: {}. {:?}", hits, current);
            hits + 1
        } else {
            hits
        };

        current += 1;
    }
}

fn is_prime(n: u32) -> bool {
    let mut current = 2;

    while current < n {
        if n % current == 0 {
            return false;
        } else {
            current += 1;
        }
    }
    true
}