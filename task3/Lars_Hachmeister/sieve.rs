fn main() {
    let a = sieb(30);
    for b in a {
        println!("{}", b);
    }
}
fn sieb(a: i32) -> Vec<i32> {

    let mut v: Vec<i32> = Vec::new();

    for i in 0..a + 1 {
        v.push(i);
    }
    for i in 2..a {
        if v[i as usize] != 0 {
            let mut b = i * 2;
            while b <= a {
                v[b as usize] = 0;
                b += i;
            }
        }

    }
    let mut res: Vec<i32> = Vec::new();
    for i in 2..a {
        if v[i as usize] != 0 {
            res.push(v[i as usize]);
        }
    }
    res
}
