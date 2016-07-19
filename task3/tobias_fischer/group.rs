fn main() {

    println!("{}", group_letter(Group::Plantex));
    println!("{}", group_letter(Group::AvzRun));
    println!("{}", group_letter(Group::SpaceGame));
}

enum Group {
    Plantex,
    AvzRun,
    SpaceGame,
}

fn group_letter(g: Group) -> char {
    match g {
        Group::AvzRun => 'a',
        Group::SpaceGame => 'b',
        Group::Plantex => 'c',
    }
}
