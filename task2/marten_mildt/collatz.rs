fn main() {

    for i in 1..21 {
        println!("{} -> {}", i, collatz(i));
    }
}

fn collatz(mut a: i32) -> i32 {
    let mut n = 0;

    while a > 1 {

        a = if a % 2 == 0 {
            a / 2
        } else {
            3 * a + 1
        };

        n += 1;
    }

    n
}
