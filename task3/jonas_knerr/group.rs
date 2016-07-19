
fn main() {
    println!("Gruppe {:?}", group_letter("Plantex"));
}
fn group_letter(a: &str) -> Option<char> {
    match a {
        "AVZRun" => Some('a'),
        "Plantex" => Some('b'),
        "SpaceGame" => Some('c'),
        _ => None,
    }

}
