fn main() {
    for i in 1..21 {
        println!("Number {} requires {} iterations to complete.",i,collatz(i));
    }
}
fn collatz(mut n: i32) -> i32 {
    let mut cnt = 0;
    while n != 1 {
        n = if n % 2 == 0 {
            n/2
        } else {
            n*3+1
        };
        cnt += 1
    }
    cnt
}

