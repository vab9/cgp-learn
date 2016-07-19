fn main() {
    println!("{:?}", eratosthenes(40));
}

fn eratosthenes(n: i32) -> Vec<i32> {
    let mut v = Vec::new();
    for x in 2..n {
        let mut found = false;
        for t in 2..x - 1 {
            if x % t == 0 {
                found = true;
                continue;
            }
        }
        if !found {
            v.push(x);
        }
    }
    v
}
