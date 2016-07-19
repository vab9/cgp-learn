enum Error { NotAValidInput }

fn main() {
    let pl = "Plantex";
    let av = "AVZ-Run";
    let sp = "Space Game";

    match group_letter(pl) {
        Ok(v) => println!("Plantex group => {}", v),
        Err(_e) => println!("Error => invalid input"),
    }
    match group_letter(av) {
        Ok(v) => println!("Plantex group => {}", v),
        Err(_e) => println!("Error => invalid input"),
    }
    match group_letter(sp) {
        Ok(v) => println!("Plantex group => {}", v),
        Err(_e) => println!("Error => invalid input"),
    }
    match group_letter("") {
        Ok(v) => println!("Plantex group => {}", v),
        Err(_e) => println!("Error => invalid input"),
    }
}

fn group_letter(group_name: &str) -> Result<char,Error> {
    match group_name.as_ref() {
        "Plantex" => Ok('c'),
        "AVZ-Run" => Ok('a'),
        "Space Game" => Ok('b'),
        _ => Err(Error::NotAValidInput),
    }
}
