fn group_letter(name: &str) -> char {
    match name {
        "Plantex" => 'c',
        "AVZ-Run" => 'a',
        "Space Game" => 'b',
        _ => 'x',
    }
}




fn main() {
    println!("Plantex -> {}", group_letter("Plantex"));
    println!("AVZ-Run -> {}", group_letter("AVZ-Run"));
}
