fn main() {
    let pl = "Plantex";
    let av = "AVZ-Run";
    let sp = "Space Game";

    println!("Plantex group => {}", group_letter(pl));
    println!("AVZ group => {}", group_letter(av));
    println!("Space Game group => {}", group_letter(sp));
    println!("Anything else => {}", group_letter(""));
}

fn group_letter(group_name: &str) -> char {
    match group_name.as_ref() {
        "Plantex" => 'c',
        "AVZ-Run" => 'a',
        "Space Game" => 'b',
        _ => '0',
    }
}