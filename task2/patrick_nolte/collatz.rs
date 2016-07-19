fn main() {
    for x in 1..20 {
        println!("{} -> {}", x, collatz(x));
    }
}

fn collatz(mut n: i32) -> i32 {
    let mut count = 0;
    while n != 1 {
        count += 1;
        n = if n % 2 == 0 {
            n / 2
        } else {
            n * 3 + 1
        }
    }
    count
}
