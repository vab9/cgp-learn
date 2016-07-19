fn main() {
    println!("Primzahltest: ");
    let mut i = 1;
    for n in 2.. {
        if is_prime(n) {
            println!("{} ist eine Primzahl.", n);
            i = i + 1;
        }
        if i == 20 {
            break;
        }
    }
}

fn is_prime(n: i32) -> bool {

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
