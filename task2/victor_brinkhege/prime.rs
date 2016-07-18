fn main() {
    let mut primes = 0;

    for i in 2.. {
        let result = prime(i);

        if result {
            println!("{:?} -> {:?}", primes + 1, i);
            primes += 1;
        }
        if primes == 20 {
            return;
        }
    }

}

fn prime(x: i32) -> bool {

    if x % 2 == 0 || x % 3 == 0 {
        return false;
    }

    let mut i = 5;

    let limit = (x as f64).sqrt() as i32;

    while i < limit {
        if x % i == 0 {
            return false;
        }
        i += 2;
    }
    return true;

}
