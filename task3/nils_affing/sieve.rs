fn main() {
    let vec = siev(70);

    println!("Ergebnis des Siebs: ");

    for element in vec {
        println!("{}", element);
    }
}

fn siev(n: i32) -> Vec<i32> {
    let mut siev = Vec::new();

    'outer: for current in 2..n {
        for factor in 2..current - 1 {
            if current % factor == 0 {
                siev.push(false);
                continue 'outer;
            }
        }
        siev.push(true);
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
