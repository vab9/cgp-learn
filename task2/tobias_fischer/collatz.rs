fn main() {
    for n in 1..21 {
        println!("N = {} Iterationen {}", n, collatz(n));
    }
}

fn collatz(mut x: u32) -> u32 {
    let mut i = 0;

    while x != 1 {
        x = if x % 2 == 0 {
            x / 2
        } else {
            3 * x + 1
        };
        i += 1;
    }
    i
}
