fn get_prim(n: usize) -> Vec<u32> {

    let mut v = Vec::with_capacity(n);
    let r = n as u32;
    for x in 1..r + 1 {
        v.push(x);
    }

    let wurzel = (n as f32).sqrt();

    for i in 2..(wurzel as u32) {
        for j in 2..n {
            if v[j] % i == 0 && v[j] != i {
                v[j] = 0;
            }
        }
    }

    let mut vec = Vec::new();

    for y in 0..v.len() {
        if v[y] != 0 && v[y] != 1 {
            vec.push(v[y]);
        }
    }

    vec

}

fn main() {

    let n = 20;

    let v = get_prim(n);

    for i in 0..v.len() {
        println!("{}", v[i]);
    }
}
