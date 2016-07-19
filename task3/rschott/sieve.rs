fn main() {
    let mut v = Vec::new();
    v = strike(233);
    for e in v {
        if e != 0 {
            println!("{}", e);
        }
    }

}

fn strike(n: i32) -> Vec<i32> {
    let mut v = Vec::new();
    for i in 2..n {
        v.push(i);
    }
    for t in 2..n / 2 {
        for i in 0..v.len() as i32 {
            if v[i as usize] % t == 0 && t != v[i as usize] {
                v[i as usize] = 0;
            }
        }
    }
    v
}
