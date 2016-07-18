fn main() {

    let mut x = 1;
    while x < 21 {
        println!("{} is prime: {}", x, is_prime(x));
        x = x + 1;
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
            i = i + 6
        }

        true
    }

}
