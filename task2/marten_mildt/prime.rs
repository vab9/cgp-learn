fn main() {
    let mut n = 1;
    let mut i = 0;

    while i < 20 {
        if is_prime(n) {
            println!("{}", n);
            i += 1;
        }
        n += 1;
    }
}

fn is_prime(n: u32) -> bool {

    if n <= 3 {
        true
    } else if n % 2 == 0 || n % 3 == 0 {
        false
    } else {
        for i in 4..n / 2 {
            if n % i == 0 {
                return false;
            }
        }

        true
    }
}
