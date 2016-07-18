fn main() {
    // collatzen
    for i in 1..21 {
        println!("{:?} -> {:?}", i, collatz(i));
    }
}

fn collatz(mut x: i32) -> i32 {
    let mut count = 0;

    while x > 1 {
        count += 1;
        x = if x % 2 == 0 {
            x / 2
        } else {
            x * 3 + 1
        }
    }

    count
}
