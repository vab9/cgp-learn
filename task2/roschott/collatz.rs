fn main() {
    println!("Collatz: ");
    for x in 1..20 {
        println!("{} -> {}", x, collatz(x));
    }
}


fn collatz(n: i32) -> i32 {
    let mut x = n;
    let mut i = 0;
    while x != 1 {
        x = if x % 2 == 0 {
            x / 2
        } else {
            3 * x + 1
        };
        i = i + 1;
    }
    (i)
}
