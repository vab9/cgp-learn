fn main() {
    for i in 1..21 {
        println!("{} -> {}", i, collatz(i));
    }

}

fn collatz(mut m: i32) -> i32 {
    let mut count = 0;
    while m > 1 {
        m = if m % 2 == 0 {
            m / 2
        } else {
            3 * m + 1
        };
        count += 1
    }
    count
}
