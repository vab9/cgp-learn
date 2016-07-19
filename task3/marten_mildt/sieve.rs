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

    let fsqrt = (n as f64).sqrt();
    let nsqrt = fsqrt as usize;

    for i in 2..nsqrt {
        if !gestrichen[i - 2] {
            primes.push(i);

            let mut j: usize = i * 2;

            // streiche die vielfachen von i, beginnend mit i*2
            while j < (n - 1) {
                gestrichen[j - 2] = true;

                j += i;
            }
        }
    }

    for i in (nsqrt + 1)..(n - 1) {
        if !gestrichen[i - 2] {
            primes.push(i);
        }
    }

    primes
}
