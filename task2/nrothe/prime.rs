fn main() {
    let mut i = 0;
    let mut count = 0;
    while count < 20 {
        if is_prime(i) {
            println!("{}", i);
            count += 1;
        }
        i += 1;
    }

}

fn is_prime(n: i32) -> bool {
    let mut isprime: bool = true;
    if n > 0 && n < 4 {
        true
    } else if n == 4 || n == 0 {
        false
    } else {
        for i in 2..n / 2 {
            if n % i == 0 {
                return false;
            }
        }
        true
    }
}
