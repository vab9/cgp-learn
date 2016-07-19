fn main() {
    let mut i = 1;
    let mut j = 0;
    while i <= 20 {
        if is_prime(j) {
            println!("{}", j);
            i += 1;
        }
        j += 1;
    }
}

fn is_prime(n: u32) -> bool {
    let mut is = true;
    if n == 0 || n == 1 {
        is = false;
    } else {
        for i in 2..n {
            if n % i == 0 {
                is = false;
            }
        }
    }
    is
}
