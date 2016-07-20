fn main() {
    println!("{:?}", sieve(40));
}
fn sieve(n: u32) -> Vec<u32> {
    let mut v: Vec<u32> = Vec::new();
    let mut result: Vec<u32> = Vec::new();

    for i in 2..n + 2 {
        v.push(i);
    }

    for t in 2..n + 1 {
        let mut tmp = 2 * t;

        while tmp < n {
            v[tmp as usize] = 0;
            tmp = tmp + t;
        }
    }

    for e in v {
        if e != 0 {
            result.push(e);
        }
    }
    result

}
