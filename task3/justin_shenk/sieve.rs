// Sieve of Eratosthenes
fn main() {
    let vec = vec![1,2,3,5,7,11,13,17,19];
    assert_eq!(vec, return_primes(20));
    println!("{:?}", return_primes(20));
}
// Iterate through all numbers greater than 3 and not divisible by 2 or 3
fn is_prime(n: u32) -> bool {
    if n < 4 {
        return true;
    }
    if n % 2 == 0 || n % 3 == 0 {
        return false;
    }
    for i in 5 .. n {
        if n % i == 0 {
            return false;
        }
    }
    true
}

// Find all primes less than n
fn return_primes(n: u32) -> Vec<u32> {
    let mut primes = (1..n+1).collect::<Vec<u32>>();
    primes.retain(|number| is_prime(*number));
    return primes
}
