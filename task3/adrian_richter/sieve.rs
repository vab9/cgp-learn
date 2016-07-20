fn main() {
    for p in get_sieve(100) {
        println!("{}", p);
    }
}

fn get_sieve(n: u32) -> Vec<u32> {
    let mut list = Vec::new();
    let mut primes = Vec::new();

    if n >= 2 {
        list.push(false);
        list.push(false);
        for _ in 2..n {
            list.push(true);
        }

        for t in 2..n {
            if list[t as usize] {
                primes.push(t);
            }
            for i in t..n {
                if i % t == 0 {
                    list[i as usize] = false;
                }
            }
        }
    }
    primes
}
