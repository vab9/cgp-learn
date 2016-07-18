fn main() {
    let mut count = 20;
    let mut iter = 0;
    while count > 0 {
        iter = iter + 1;
        let a = is_prime(iter);
        if a.1 == true {
            count = count - 1;
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
