fn main() {
    println!("{}", group_letter("Plantex"));
}

fn group_letter(name: &str) -> char {
    match name {
        "Plantex" => 'a',
        "AVZ-Run" => 'b',
        "Space Game" => 'c',
        _ => '?',
    }
}
