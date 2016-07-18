/// Prime Number test for first 20 prime numbers.
///
/// Written by Lukas Kalde, Dennis Lindner

fn main() {
    let mut i = 1;
    let mut prime_n = 0;
    while prime_n < 20 {
        if is_prime(i) {
            println!("{}. {} is a prime number", prime_n, i);
            prime_n = prime_n + 1;
        }
        i += 1;
    }
}

/// check if prime number.
///
/// Parameter n has to be positive integer, return value is bool.
fn is_prime(n: i32) -> bool {
    if n <= 1 {
        false
    } else if n <= 3 {
        true
    } else if n % 2 == 0 || n % 3 == 0 {
        false
    } else {
        let mut i = 5;
        while i * i <= n {
            if n % i == 0 || n % (i + 2) == 0 {
                return false;
            }
            i = i + 6;
        }
        true
    }
}
