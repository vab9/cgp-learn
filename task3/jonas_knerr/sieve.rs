fn main() {
    println!("{:?}", sieve(40));
}
fn sieve(n: u32) -> Vec<u32> {
    let mut v: Vec<u32> = Vec::new();
    let mut result: Vec<u32> = Vec::new();
    let mut tmp;

    for i in 2..n + 2 {
        v.push(i);
    }

    for t in 2..n + 1 {
        // println!("T :{}", t);
        tmp = t;
        tmp = tmp + t;
        while tmp < n {
            //    println!("{}", tmp);
            v[tmp as usize] = 0;
            tmp = tmp + t;

        }
    }
    //   println!("{:?}", v);
    for e in &v {
        if *e != 0 {
            result.push(*e);
        }
    }
    result

}
