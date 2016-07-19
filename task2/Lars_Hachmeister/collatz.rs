fn main() {
    for i in 1..21 {
        let b = collatz(i);
        println!("Zahl: {} Anzahl der Schritte: {}", i, b)
    }
}

fn collatz(a: i32) -> i32 {
    let mut b = 0;
    let mut c = a;
    while c > 1 {
        if c % 2 == 0 {
            c = c / 2;
            b += 1
        } else {
            c = c * 3 + 1;
            b += 1;
        }
    }
    b
}
