fn collatz(mut n: i32) -> i32 {
    let mut x = 0;
    while n > 1 {
        n = if n % 2 == 0 {
            n / 2
        } else {
            3 * n + 1
        };
        x += 1;
    }
    x
}

fn main() {
    for i in 1..21 {
        println!("{} -> {}", i, collatz(i));
    }
}
