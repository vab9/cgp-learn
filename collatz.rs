fn main() {
    for i in 1..21 {
        let b = collatz(i);
        println!("Zahl: {} Iterationen: {}", i, b)
    }
    // let b = 3;
    // let c = collatz(b);
    // println!("{} -> {}", b, c)
}

fn collatz(i: i32) -> i32 {
    let mut n = i;
    let mut z = 0;
    while n > 1 {
        if n % 2 != 0 {
            z = z + 1;
            n = n * 3 + 1;
        } else {
            z = z + 1;
            n = n / 2;
        }
    }
    z
}
