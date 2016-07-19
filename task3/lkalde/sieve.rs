/// Print first 100 Prime Numbers

fn main() {
    let n = 100;
    let prime = sieve(n);
    for i in 1..prime.len() {
        println!("{} is a prime number", prime[i as usize]);
    }

}


/// Implement sieve of Eratosthenes
fn sieve(n: u32) -> Vec<u32> {
    let mut numbers = Vec::new();   //Hold true for index if possible prime number

    // calculate upper boarder to which we need to count in the sieve
    let b = (n as f32).sqrt().ceil() as u32;

    // assume all numbers as prime numbers
    for _ in 0..n + 1 {
        numbers.push(true);
    }
    numbers[1] = false;  //by definition



    let mut start;  //initial value for sieve iteration
    let mut j;      //current number that is to be scratched

    // execute actual sieve
    for i in 2..b + 1 {
        // only need to scratch multiple of i if i has not been scratched yet
        if numbers[i as usize] == true {
            start = i;
            j = 2 * i;
            // scracth all multiples
            while j <= n {
                numbers[j as usize] = false;
                j += start;
            }
        }
    }



    // prepare return vector
    let mut ret = Vec::new();
    for i in 0..n + 1 {
        if numbers[i as usize] == true {
            ret.push(i);
        }
    }

    ret

}
