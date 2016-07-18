fn main() {
    for i in 1..21 {
        println!("{} -> {}", i, collatz(i));
    }
}

fn collatz(mut n: u32) -> u32 {
    let mut i = 0;
    while n > 1 {
        n = if n % 2 == 0 {
            n / 2
        } else {
            3 * n + 1
        };
        i += 1;
    }
    i
}
