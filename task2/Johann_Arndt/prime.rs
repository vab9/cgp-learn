fn main() {

    let mut it = 0;
    let mut j = 0;
    while it < 20 {
        j += 1;
        it = if is_prime(j) {

            println!("{}", j);
            it + 1
        } else {
            it
        };
    }
}

fn is_prime(n: u32) -> bool {

    let mut i = 1;
    let mut back = true;

    while i < n / 2 && back == true {
        i += 1;
        back = if n % i != 0 {
            true
        } else {
            false
        };

    }

    back
}
