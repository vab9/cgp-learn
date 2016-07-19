fn main() {
    println!("Primzahlen bis 50: {:?}", erastosthenes(50))
}

fn erastosthenes(n: u32) -> Vec<u32> {
    let mut prime = Vec::new();
    prime.push(false);
    prime.push(false);
    for _ in 2..n + 1 {
        prime.push(true);
    }

    for zeiger in 2..n / 2 {
        if prime[zeiger as usize] == true {
            let mut x = zeiger + zeiger;
            while x <= n {
                prime[x as usize] = false;
                x = x + zeiger;
            }
        }
    }

    let mut result = Vec::new();
    for i in 0..n + 1 {
        if prime[i as usize] == true {
            result.push(i);
        }
    }
    result
}
