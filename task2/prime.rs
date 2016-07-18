fn main() {
    let mut i = 0;
    let mut num = 0;
    while i < 20 {
        if is_prime(num){
        println!("{:?}",num );
        i +=1;
        };
        num += 1;
    }
}
fn is_prime(n : u16) -> bool {
    if n == 0 || n == 1 {
        return false;
    }
    for i in 2..n {
        if n % i == 0 {
            return false;
        }
    }
    true

}
