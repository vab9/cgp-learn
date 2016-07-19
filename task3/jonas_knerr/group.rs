
fn main() {
    println!("Gruppe {}", group_letter("Plantex"));
}
fn group_letter(a: &str) -> &str {
    if a == "AVZRun" {
        "a"
    } else if a == "Plantex" {
        "b"
    } else if a == "SpaceGame" {
        "c"
    } else {
        "Error: Unknow Group"
    }
}
