fn main() {
    let tmp = eratosthenes(100);
    for i in &tmp {
        println!("{}", i);
    }
}

fn eratosthenes(n: usize) -> Vec<usize> {
    let mut v = Vec::new();
    for i in 2..n + 1 {
        v.push(i);
    }
    let mut elem = 2;
    while elem < v.len() {
        for i in elem..v.len() {
            if v[i] % elem == 0 {
                v[i] = 0;
            }
        }
        elem += 1;
    }
    let mut i = 0;
    while i < v.len() {
        if v[i] == 0 {
            v.remove(i);
        } else {
            i += 1;
        }
    }
    v
}
