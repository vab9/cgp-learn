fn main() {

    let mut x = 1;
    let mut i = 0;
    while i < 20 {
        if is_prime(x) {
            println!("{} is prime", x,);
            i += 1;
        }
        x += 1;
    }

}

fn is_prime(n: u32) -> bool {
    if n <= 1 {
        false
    } else if n <= 3 {
        true
    } else if n % 2 == 0 || n % 3 == 0 {
        false
    } else {
        let mut i = 5;
        while i * i <= n {
            if n % i == 0 || n % (i + 2) == 0 {
                return false;

            }
            i += 6;
        }

        true
    }

}
