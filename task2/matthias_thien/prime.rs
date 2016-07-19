fn main() {
    let mut n = 0;
    let mut i = 0;
    while i < 20 {
        if is_prime(n) {
            println!("{}", n);
            i += 1;
        }
        n += 1;
    }
}

fn is_prime(n: i32) -> bool {
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
