fn main() {

    for x in 2..21 {
        println!("{} -> {}", x, prim(x));
    }

}

fn prim(n: i32) -> bool {

    for teiler in 2..n {

        if n % teiler == 0 {
            return false;

        }
    }

    return true;
}
