fn main() {
    let result = streichen(100);
    for t in &result {
        println!("{}", t);
    }
}
fn streichen(n: usize) -> Vec<usize> {
    let mut vec = Vec::new();
    let mut del = Vec::new();
    let mut result = Vec::new();
    for i in 2..n + 1 {
        vec.push(i);
        del.push(false);
    }
    for t in 2..n + 1 {
        for i in 0..vec.len() {
            if vec[i] != t {
                if vec[i] % t == 0 {
                    del[i] = true;
                }
            }
        }
    }
    let mut i = 0;
    while i < vec.len() {
        if del[i] == false {
            result.push(vec[i]);
            i += 1;
        } else {
            i += 1;
        }
    }
    result
}
