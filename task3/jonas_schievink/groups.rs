fn main() {
    println!("{:?}", group_letter("Plantex"));
}

fn group_letter(name: &str) -> Option<char> {
    match name {
        "AVZ-Run" => Some('a'),
        "Space Game" => Some('b'),
        "Plantex" => Some('c'),
        _ => None,
    }
}
