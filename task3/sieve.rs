fn main() {
    let n = 100;
    let size = n - 2;
    let mut vec: Vec<u32> = Vec::with_capacity(size);

    for i in 2..n {
        let x = i as u32;
        vec.push(x);
    }
    vec = sieve(vec);
    for i in vec {
        println!("Prim number => {}", i);
    }
}

fn sieve(mut vec: Vec<u32>) -> Vec<u32> {

    let n = vec.len();
    for i in 2..n + 1 {
        let m = i as u32;
        vec.retain(|&x| (x % m != 0 || x == m));
    }
    vec
}