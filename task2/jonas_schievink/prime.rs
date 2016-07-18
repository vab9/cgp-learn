fn is_prime(n: u64) -> bool {
    if n == 1 {
        return false;
    }

    for i in 2..n/2+1 {
        if n % i == 0 {
            return false;
        }
    }

    true
}

fn main() {
    for i in 1..21 {
        println!("{} -> {}", i, if is_prime(i) { "prime" } else { "not prime" });
    }
}
