
fn main() {
    println!("Gruppe {}", group_letter("Plantex"));
}
fn group_letter(a: &str) -> char {
    match a {
        "AVZRun" => 'a',
        "Plantex" => 'b',
        "SpaceGame" => 'c',
        _ => 'x',
    }

}
