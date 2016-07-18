fn main() {


    for j in 1..20 {
        println!("{}", collatz(j));
    }

}

fn collatz(n: u32) -> u32 {

    let mut i = n;
    let mut it = 0;

    while i != 1 {
        i = if i % 2 == 0 {
            i / 2
        } else {
            i * 3 + 1
        };

        it += 1;
    }

    it

}
