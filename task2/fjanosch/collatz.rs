fn main() {
    for elem in 1..21 {
        println!("{} -> {}", elem, collatz(elem));
    }
}

fn collatz(m: i32) -> i32 {
    let mut n = m;
    let mut count = 0;
    while n != 1 {
        n = if n % 2 == 0 {
            count = count + 1;
            n / 2
        } else {
            count = count + 1;
            n * 3 + 1
        }
    }
    count
}
