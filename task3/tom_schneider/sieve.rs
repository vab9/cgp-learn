
fn main() {
    let v = eras(100);
    for c in &v {
        println!("{}", c);
    }
}


fn eras(n: u32) -> Vec<u32> {
    let mut arr = vec![false; (n+1) as usize];
    let mut v = Vec::new();

    for t in 2..(n + 1) {
        if !arr[t as usize] {
            v.push(t);
            let mut i = t + t;
            while i <= n {
                arr[i as usize] = true;
                i = i + t;
            }
        }
    }
    v
}
