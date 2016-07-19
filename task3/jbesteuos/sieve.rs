fn main() {
    println!("{:?}", sieve());
}

fn sieve() -> Vec<usize> {

    let mut array: [usize; 100] = [0; 100];
    for x in 0..100 {
        array[x] = x;
    }

    for t in 2..100 {
        for n in 2..100 {
            if t * n >= 100 {
                break;
            } else {
                array[t * n] = 0;
            }
        }
    }
    let mut v = vec![];

    for x in 0..100 {
        if array[x] != 0 {
            v.push(x);
        }
    }
    v
}
