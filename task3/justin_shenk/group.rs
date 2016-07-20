fn main() {

    println!("{:?}", group_letter("Plantex"));

}

fn group_letter(g: &str) -> Option<char> {
    match g {
        "Plantex" => Some('a'),
        "AVZ-Run" => Some('b'),
        "Space Game" => Some('c'),
        _ => None,
    }
}
