fn main() {

    println!("{:?}", group_letter("Plantex"));
}

fn group_letter(n: &str) -> Option<char> {

    match n {
        "Plantex" => Some('C'),
        "AVZ-Run" => Some('A'),
        "Space Game" => Some('B'),
        _ => None,
    }
}
