fn main() {
    let result = group_letter("Plantex");

    match result {
        Ok(v) => println!("Gruppe: {}", v),
        Err(e) => println!("{}", e),
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
