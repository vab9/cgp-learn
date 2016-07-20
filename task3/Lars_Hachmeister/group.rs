

fn main() {
    let a = group_letter(Group::Plantex);
    println!("{}", a);
}

enum Group {
    AvzRun,
    SpaceGame,
    Plantex,
}

fn group_letter(name: Group) -> char {
    match name {
        Group::AvzRun => 'A',
        Group::SpaceGame => 'B',
        Group::Plantex => 'C',
    }
}
