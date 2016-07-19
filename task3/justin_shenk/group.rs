fn main() {
    println!("{}",group_letter("Plantex"));
}

fn group_letter(g:&str) -> char {

    match g {
        "Plantex" => 'a',
        "AVZ-Run" => 'b',
        "Space Game" => 'c',
        _ => '',
    }
}
