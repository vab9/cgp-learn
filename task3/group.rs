fn main() {
    println!("{}", group_letter("Plantex"));
}

fn group_letter(name: &str) -> char {
    match name {
        "AVZ Run" => 'a',
        "Marble Folly" => 'b',
        "Plantex" => 'c',
        "Space Shooter" => 'd',
        _ => 'e',
    }
}
