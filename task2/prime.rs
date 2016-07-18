fn main() {
    for i in 1..21 {
        let a = prime(i);
        println!("Zahl:{}  Primzahl? {}", i, a)

    }

}

fn prime(a: i32) -> bool {
    let mut b = a;
    let c = b - 1;
    let mut e = true;
    for i in 2..c {
        if b % i == 0 {
            e = false;
        };
    }
    e
}
