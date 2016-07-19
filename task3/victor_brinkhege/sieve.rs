
fn sieve(number: usize) -> Vec<usize> {

    let n = number + 1;
    let mut primes: Vec<bool> = vec![true; n];
    primes[0] = false;
    primes[1] = false;
    for i in 2..n {
        for j in 2..n {
            if i != j {
                if j % i == 0 {
                    primes[j] = false;
                }
            }

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
