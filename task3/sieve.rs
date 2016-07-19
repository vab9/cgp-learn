fn main() {
    let v = sieve(100);
    for i in v {
        println!("{:?}", i);
    }
}


fn sieve(n: i32) -> Vec<i32> {
    let mut work = Vec::new();
    // Vector mit allen Zahlen bis n
    for i in 0..n + 1 {
        work.push(i);
    }

    work[1 as usize] = 0;
    for iter in 2..n {
        if work[iter as usize] != 0 {
            let mut demo = 2 * iter;
            while demo < n {
                work[demo as usize] = 0;
                demo += iter;
            }
        }
    }
    let mut ergebnis = Vec::new();
    for iter in &work {
        // ohne & würde man den Vector in die Schleife schieben
        // so nur eine Referenz(richtiges Wort?)
        if work[*iter as usize] != 0 {
            // * hier um zu dereferenzieren , da "iter" hier
            // eine Referent auf ein i32 ist, also ein &i32
            ergebnis.push(work[*iter as usize]);
        }
    }

    // ALT
    // for iter in 2..n {
    //     if work[iter as usize] != 0 {
    //         ergebnis.push(work[iter as usize]);
    //     }
    // }

    ergebnis
}
