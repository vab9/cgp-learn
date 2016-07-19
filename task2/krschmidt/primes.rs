use::std::io;

fn main() {

    println!("Type in the maximum number you want to check if prime:");

    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .expect("failed to read line");

    let n: i32 = input.trim().parse()
        .expect("Please type a number!");

    for i in 0..n+1 {
        if is_prime(i) {
            print!("{} ", i);
        }
    }
    println!("");
}


fn is_prime(n: i32) -> bool {
    !dividable(n, 2)
}

fn dividable(n: i32, d: i32) -> bool {
    if n < 2*d {
        false
    } else {
        n%d == 0 || dividable(n, d+1)
    }
}
