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

    let mut i: u32 = 1;

    while i < (n / 2) && n % (i + 1) != 0 {
        i += 1;
    }

    if i < (n / 2) {
        false
    } else {
        true
    }

}
