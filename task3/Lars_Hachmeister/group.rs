

fn main() {

    let a = group_letter(Group::Plantex);
    println!("{}", a)
}

enum Group {
    AvzRun,
    SpaceGame,
    Plantex,
}

fn group_letter(name: Group) -> char {
    let letter = match name {
        Group::AvzRun => 'A',
        Group::SpaceGame => 'B',
        Group::Plantex => 'C',
    };
    letter




    // let a = if name == "AVZ-Run" {
    //     "A"
    // } else if name == "Space Game" {
    //     "B"
    // } else if name == "Plantex" {
    //     "C"
    // } else {
    //     "Z"
    // }; // bei falscher eingabe
    // a
}
