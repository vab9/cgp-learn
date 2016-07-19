/// Prints first 20 prime numbers
fn main() {
    let mut cnt = 0;
    let mut n = 0;
    while cnt < 20 {
        n += 1;
        if is_prime(n){
            cnt += 1;
            println!("{}: {} is prime", cnt, n);
        }
    }
}
fn is_prime(n: i32) -> bool {
    // If number is divisible by a number greater than 1 and less than itself
    for i in 2..n/2 + 1 {
        if n % i == 0 {
            // then return false
            return false;
        }
    }
    // Else the number is prime
    true
}
