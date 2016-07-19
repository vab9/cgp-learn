fn main() {

    for i in sieb(100) {
        println!("{}", i);
    }
}

fn sieb(n: u32) -> Vec<u32> {
    let mut liste = vec![true; n as usize];
    let mut i = 2;

    while i + 1 < liste.len() {
        if liste[i as usize] {
            for j in i + 1..liste.len() {
                if j % i == 0 {
                    liste[j] = false;
                }
            }
        }
        i += 1;
    }

    let mut res = Vec::new();
    for k in 1..n {
        if liste[k as usize] {
            res.push(k);
        }
    }
    res
}
