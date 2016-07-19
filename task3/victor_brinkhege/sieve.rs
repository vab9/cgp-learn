
fn sieve(number: usize) -> Vec<usize> {

    let n = number + 1;
    let mut primes = vec![true; n];
    primes[0] = false;
    primes[1] = false;
    for i in 2..n {
        let mut increment = 2;
        let mut multiple = i * increment;
        while multiple <= n {
            primes[multiple] = false;
            increment += 1;
            multiple = i * increment;
        }
    }

    let mut vector = Vec::new();

    for i in 0..primes.len() {
        if primes[i] {
            vector.push(i);
        }
    }

    vector

}

fn main() {
    println!("{:?}", sieve(100));
}
