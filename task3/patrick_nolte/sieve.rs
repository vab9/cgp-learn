fn main() {
    println!("{:?}", eratosthenes(40));
}

fn eratosthenes(n: u32) -> Vec<u32> {
    let mut v = Vec::new();
    for i in 1..n {
        v.push(true);
    }
    for i in 1..n - 1 {
        for j in 2..i - 1 {
            if i % j == 0 {
                v[(i) as usize] = false;
                break;
            }
        }
    }
    let mut result = Vec::new();
    for i in 0..(v.len() - 1) {
        if v[i as usize] {
            result.push(i as u32);
        }
    }
    result
}
