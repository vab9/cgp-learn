fn collatz(mut n: i32) -> i32 {
    
    let mut iter = 0;
    
    while n != 1 {

        n = if n % 2 == 0 {
            n / 2
        } else {
            n * 3 + 1
        };
       
        iter += 1; 
        
    }

    iter
}

fn main() {
    
    for i in 1..21 {
            println!("{} -> {}", i, collatz(i));
    }
}
