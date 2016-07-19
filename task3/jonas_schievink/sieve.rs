fn main() {
    let result = sieve(100);
    println!("{:?}", result);

    for i in 2..result.len() {
        if result[i] {
            println!("{} is prime", i);
        }
    }
}

/// Gibt einen Vektor zurück, der für jede Zahl an deren Index speichert ob sie prim ist.
fn sieve(n: usize) -> Vec<bool> {
    let mut sieve = vec![true; n];

    // Manuell 0 und 1 als nicht-prim markieren
    sieve[0] = false;
    sieve[1] = false;

    for t in 2..n + 1 {
        // Streiche alle Vielfache, beginnend bei 2 * t
        for i in 2.. {
            let index = i as usize * t as usize;

            if index < sieve.len() {
                sieve[index] = false;
            } else {
                break;
            }
        }
    }

    sieve
}
