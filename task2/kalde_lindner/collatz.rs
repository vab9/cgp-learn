/// Implementation of Collatz in Rust.
/// Written by Lukas Kalde, Dennis Lindner
fn main() {
    for i in 1..21 {
        println!("{} -> {}", i, collatz(i));
    }
}


/// Actual collatz.
///
/// Parameter n is initial Value. Returns number of iterations.
fn collatz(mut n: u32) -> (u32) {
    let mut i = 0;

    while n != 1 {
        n = {
            if n % 2 == 0 {
                n / 2
            } else {
                n * 3 + 1
            }
        };
        i = i + 1;

    }
    i
}
