fn main() {
    for i in 1..21 {
        let b = collatz(i);
        println!("Zahl: {} Iterationen: {}", i, b);
    }
}

fn collatz(i: i32) -> i32 {
    let mut n = i;
    let mut z = 0;
    while n > 1 {
        n = if n % 2 != 0 {
            n * 3 + 1
        } else {
            n / 2
        };
        z = z + 1;
    }
    z
}
