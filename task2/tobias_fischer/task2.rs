fn main() {

    let mut n = 1;
    while n < 21 {
        println!("N = {} Iterationen {}", n, collatz(n));
        n = n+1;
    }



}

fn collatz(mut x: u32) -> (u32){
    let mut i = 0;

    while x!= 1 {
        x =
            if x % 2 == 0{
                x / 2
            }
            else{
                3 * x +1
            };
            i = i + 1;
    }
    i
}
