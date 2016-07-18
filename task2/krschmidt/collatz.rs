use::std::io;

fn main() {

    println!("Type in a number you want to calculate Collatz for:");

    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .expect("failed to read line");

    let n: i32 = input.trim().parse()
        .expect("Please type a number!");

    println!("Collatz of {} = {}", n, collatz(n));
}


fn collatz(n: i32) -> i32 {
    let mut count = 0;
    let mut n = n;
    while n > 1 {
        n = if n%2 == 0 {
            n/2
        } else {
            3 * n + 1
        };
        count += 1;
    }
    count

}
