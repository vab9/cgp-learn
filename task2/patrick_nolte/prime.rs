fn main() {
    let mut count = 0;
    let mut prime = 1;
    while count < 20 {
        if is_prime(prime) {
            println!("Primzahl: {}.:{}", count + 1, prime);
            count += 1;
        }
        prime += 1;
    }
}

fn is_prime(n: i32) -> bool {
    for x in 2..n {
        if n % x == 0 {
            return false;
        }
    }
    true
}
