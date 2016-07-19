fn main() {
    match group_letter("Plantex") {
        Ok(n) => println!("Group: {}", n),
        Err(err) => println!("Error: {}", err),
    }
}

fn group_letter(name: &str) -> Result<char, &str> {
    match name {
        "Plantex" => Ok('a'),
        "AVZ-Run" => Ok('b'),
        "Space Game" => Ok('c'),
        _ => Err("unknown group"),
    }
}
