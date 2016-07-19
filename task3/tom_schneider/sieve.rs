
fn main() {
    let v = eras(100);
    for c in &v {
        println!("{}", c);
    }
}


fn eras(n: u32) -> Vec<u32> {
    let mut arr = Vec::new();
    let mut v = Vec::new();
    for _ in 0..(n + 1) {
        arr.push(false);
    }
    for t in 2..(n + 1) {
        if arr[t as usize] != true {
            v.push(t);
            let mut i = t + t;
            while i < (n + 1) {
                arr[i as usize] = true;
                i = i + t;
            }
        }
    }
    v
}
