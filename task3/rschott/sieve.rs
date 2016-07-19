fn main() {
    for e in strike(233) {
        if e != 0 {
            println!("{}", e);
        }
    }

}

fn strike(n: u32) -> Vec<u32> {
    let mut v = Vec::new();
    for i in 2..n {
        v.push(i);
    }
    for t in 2..n / 2 {
        for i in 0..v.len() as u32 {
            if v[i as usize] % t == 0 && t != v[i as usize] {
                v[i as usize] = 0;
            }
        }
    }
    v
}
