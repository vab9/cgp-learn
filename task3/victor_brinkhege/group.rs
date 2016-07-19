fn group_letter(name: &str) -> Option<char> {
    match name {
        "Plantex" => Some('c'),
        "AVZ-Run" => Some('a'),
        "Space Game" => Some('b'),
        _ => None,
    }
}




fn main() {
    println!("Plantex -> {:?}", group_letter("Plantex"));
    println!("AVZ-Run -> {:?}", group_letter("AVZ-Run"));
}
