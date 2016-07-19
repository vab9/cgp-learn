fn main() {
    let primes: Vec<usize> = sieve(100);

    for p in &primes {
        println!("{}", *p);
    }
}

fn sieve(n: usize) -> Vec<usize> {
    let mut gestrichen: Vec<bool> = Vec::new();
    let mut primes: Vec<usize> = Vec::new();

    for i in 2..(n + 1) {
        gestrichen.push(false);
    }

    for i in 2..n / 2 {
        for j in 2..gestrichen.len() as usize + 2 {
            if j != i && j % i == 0 {
                gestrichen[j - 2] = true;
            }
        }
    }

    for i in 0..gestrichen.len() {
        if !gestrichen[i] {
            primes.push(i + 2);
        }
    }

    primes
}
