fn is_prim(x: i32) -> bool {

    let mut b = true;

    for i in 2..x / 2 + 1 {
        if x % i == 0 {
            b = false;
        }
    }

    b
}

fn main() {
   
    for i in 1..21 {
        println!("{} -> {}", i, is_prim(i));
    }

} 
