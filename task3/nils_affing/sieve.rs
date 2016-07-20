fn main() {
    let vec = siev(150);

    println!("Ergebnis des Siebs: ");

    for element in vec {
        println!("{}", element);
    }
}

fn siev(n: i32) -> Vec<u32> {
    let mut siev = Vec::new();

    for x in 0..n - 2 {
        siev.push(true);
    }
    println!("Length: {:?}", siev.len());

    let mut factor = 2;
    for current in 2..n {
        while (factor * current - 2 < siev.len() as i32) {
            siev[(factor * current - 2) as usize] = false;
            factor += 1;
        }
        factor = 2;
    }

    let mut result = Vec::new();
    let mut count = 2;

    for r in siev {
        if r {
            result.push(count);
        }
        count += 1;
    }
    result
}
