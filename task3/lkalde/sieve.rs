/// Print first 100 Prime Numbers

fn main() {
    let n = 100;
    let prime = sieve(n);
    for i in 0..prime.len() {
        println!("{} is a prime number", prime[i as usize]);
    }

}


/// Implement sieve of Eratosthenes
fn sieve(n: u32) -> Vec<u32> {
    // Since 0 and 1 are no prime numbers, numbers[i] refers to number i+2.

    // numbers will hold all numbers from 2 to n
    let mut numbers = vec![true; (n-1) as usize];

    // calculate upper boarder to which we need to count in the sieve
    let b = ((n - 2) as f32).sqrt().ceil() as u32;


    let mut start;  //initial value for sieve iteration
    let mut j;      //current number that is to be scratched

    // execute actual sieve
    for i in 0..b + 1 {
        // only need to scratch multiple of i if i has not been scratched yet
        if numbers[i as usize] == true {
            start = i + 2;
            j = 2 * start;
            // scracth all multiples
            while j <= n {
                numbers[(j - 2) as usize] = false;
                j += start;
            }
        }
    }




    // prepare return vector
    let mut ret = Vec::new();
    for i in 0..(n - 1) {
        if numbers[i as usize] {
            ret.push(i + 2);
        }
    }

    ret

}
