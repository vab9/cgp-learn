fn main() {
    let a = sieb(70);
    for b in a {
        println!("{}", b);
    }
}

fn sieb(n: u32) -> Vec<u32> {
    let mut v = Vec::new();
    for i in 0..n + 1 {
        v.push(i);
    }
    for i in 2..n {
        if v[i as usize] != 0 {
            let mut b = i * 2;
            while b <= n {
                v[b as usize] = 0;
                b += i;
            }
        }
    }
    let mut res = Vec::new();
    for i in 2..n {
        if v[i as usize] != 0 {
            res.push(v[i as usize]);
        }
    }
    res
}
