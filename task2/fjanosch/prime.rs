fn main() {
    let mut count = 20;
    let mut iter = 0;
    while count > 0 {
        iter += 1;
        let a = is_prime(iter);
        if a == true {
            count -= 1;
            println!("{} is prime!", iter);
        }
    }
}

fn is_prime(input: i32) -> (bool) {
    let mut rly_prime = true;
    for i in 2..input {
        if (input % i == 0) {
            return false;
        }
    }
    rly_prime
}
