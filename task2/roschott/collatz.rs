fn main() {
    println!("Collatz: ");
    for x in 1..20 {
        println!("{} -> {}", x, collatz(x));
    }
}


fn collatz(mut n: i32) -> i32 {
    let mut i = 0;
    while n != 1 {
        n = if n % 2 == 0 {
            n / 2
        } else {
            3 * n + 1
        };
        i += 1;
    }
    i
}
