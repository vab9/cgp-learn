fn main() {
    let mut b = 0;
    for a in 1.. {
        if prime(a) {
            println!("{}", a);
            b += 1;
            if b == 20 {
                break;
            }
        }
    }
}

fn prime(a: i32) -> bool {
    let mut b = a;
    let c = b - 1;
    for i in 2..c {
        if b % i == 0 {
            return false;
        }
    }
    true
}
