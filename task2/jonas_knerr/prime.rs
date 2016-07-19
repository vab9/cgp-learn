fn main() {

    let mut cnt = 0;
    let mut n = 0;
    while cnt < 20 {
        n += 1;
        if is_prime(n) {
            cnt += 1;
            println!("{} : {} ist Prim", cnt, n);
        }


    }
}
fn is_prime(n: i32) -> bool {

    for i in 2..n / 2 + 1 {
        if n % i == 0 {
            return false;
        }
    }
    true

}
