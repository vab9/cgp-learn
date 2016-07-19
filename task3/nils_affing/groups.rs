fn main() {
    let result = group_letter("Plantex");

    if result.is_ok() {
        println!("Gruppe: {}", result.unwrap());
    } else {
        println!("Keine Gruppe gefunden!");
    }
}

fn group_letter(name: &str) -> Result<char, &str> {
    match name {
        "Space Game" => Ok('A'),
        "AVZ-Run" => Ok('B'),
        "Plantex" => Ok('C'),
        _ => Err("Keine Gruppe gefunden!"),
    }
}
