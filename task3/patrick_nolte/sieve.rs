fn main() {
    println!("{:?}", eratosthenes(40));
}

fn eratosthenes(n: u32) -> Vec<u32> {
    let mut v = Vec::new();
    for x in 1..n {
        v.push(x);
    }
    let mut count = 0;
    while count < v.len() {
        if v[count as usize] % 2 == 0 {
            v.remove(count);
        } else {
            count += 1;
        }
    }
    v
}
