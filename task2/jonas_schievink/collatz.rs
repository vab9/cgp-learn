fn collatz(mut n: u64) -> u64 {
    let mut iterations = 0;
    while n > 1 {
        n = if n % 2 == 0 {
            n / 2
        } else {
            n * 3 + 1
        };

        iterations += 1;
    }

    iterations
}

fn main() {
    for i in 1..21 {
        println!("{} -> {}", i, collatz(i));
    }
}
