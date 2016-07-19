fn main() {

    println!("{:?}", eratosthenes(20));
}


fn eratosthenes(n: i32) -> Vec<i32> {

    let mut numbers = vec![0; n as usize];
    for (i, j) in (0..n).enumerate() {
        numbers[i] = j;
    }

    for t in 2..n {
        for x in &mut numbers {
            if *x % t == 0 && *x != t {
                *x = 0;
            }
        }
    }

    let mut primes: Vec<i32> = vec![];
    for x in &mut numbers {
        if *x != 0 {
            primes.push(*x);
        }
    }
    primes
}
