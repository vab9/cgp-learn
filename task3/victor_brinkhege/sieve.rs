
fn sieve(n: usize) -> Vec<usize> {
    let mut vec = Vec::new();
    let mut array = [2; n + 1];
    for i in 2..n + 1 {
        vec.push(i);
    }

    for i in vec.len() {

    }

    for t in vec.iter() {
        for x in vec.into_iter() {
            if x % t == 0 {
                vec.remove(x);
            }
        }
    }
    return vec;
}

fn main() {
    println!("{:?}", sieve(37));
}
