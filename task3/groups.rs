fn main() {
    println!("{:?}", group_letter(Group::AvzRun));
    println!("{:?}", group_letter(Group::SpaceGame));
    println!("{:?}", group_letter(Group::Plantex));
}

enum Group {
    Plantex,
    AvzRun,
    SpaceGame,
}

fn group_letter(x: Group) -> char {
    let letter = match x {
        Group::Plantex => 'c',
        Group::AvzRun => 'a',
        Group::SpaceGame => 'b',
    };
    letter
}
