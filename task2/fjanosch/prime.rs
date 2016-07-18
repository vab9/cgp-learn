fn main() {
    for i in 1..21 {
        let a = is_prime(i);
        if a.1 == true {
            println!("{} is prime!", a.0)
        }
    }
}

fn is_prime(input: i32) -> (i32, bool) {
    let mut rly_prime = true;
    for i in 2..input {
        if (input % i == 0) {
            rly_prime = false;
        }
    }
    (input, rly_prime)
}
