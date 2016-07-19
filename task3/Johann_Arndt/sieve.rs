fn main() {
    let n = 100;
    let liste = sieb(&vec![true; n]);

    for i in 0..n {
        if liste[i] {
            println!("{}", i);
        }
    }
}

fn sieb(todo: &Vec<bool>) -> Vec<bool> {
    let mut i = 2;
    let mut liste = todo.clone();
    while i > 1 && i + 1 < liste.len() {
        for j in i + 1..liste.len() {
            if j % i == 0 {
                liste[j] = false;
            }
        }
        i += 1;
    }

    liste
}
