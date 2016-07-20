// Sieb des Eratosthenes-Methode um Primzahlen zu bestimmen
fn main() {
    let vec = sieve(100);
    for e in vec {
        println!("{}", e);
    }
}

fn sieve(n: u32) -> Vec<u32> {
    let mut v = Vec::new();

    for i in 2..n {
        v.push(i);
    }
    for t in 2..n / 2 {
        for i in 0..v.len() {
            if v[i] % t == 0 && t != v[i] {
                v[i] = 0;   //value = 0 heißt rausgestrichen
            }
        }
    }
    let mut sol = Vec::new();
    for i in 0..v.len() {
        if v[i] != 0 {
            sol.push(v[i]);
        }
    }
    sol
}
