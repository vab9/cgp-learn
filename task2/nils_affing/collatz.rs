fn main() {
    println!("{:?}", collatz(1));
    println!("{:?}", collatz(2));
    println!("{:?}", collatz(3));
    println!("{:?}", collatz(4));
}

fn collatz(mut n: i32) -> i32 {
    let mut iter = 0;

    while n != 1 {
        iter += 1;
        n = if n % 2 == 0 {
            n / 2
        } else {
            n * 3 + 1
        };
    }
    iter
}
