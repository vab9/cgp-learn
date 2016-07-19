fn main() {
    println!("{}", group_letter("AVZ-Jump"));
}
fn group_letter(input: &str) -> char {
    match input {
        "Plantex" => 'C',
        "AVZ-Run" => 'A',
        "Space Game" => 'B',
        _ => '#',
    }
}
