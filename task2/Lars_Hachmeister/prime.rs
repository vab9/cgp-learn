fn main() {
    let mut a = 0;
    let mut b = 1;
    while a < 20 {
        if prime(b) {
            println!("{}", b);
            a = a + 1;
        }
        b = b + 1
    }
}

fn prime(a: i32) -> bool {
    let mut b = a;
    let c = b - 1;
    let mut e = true;
    for i in 2..c {
        if b % i == 0 {
            e = false;
        };
    }
    e
}
