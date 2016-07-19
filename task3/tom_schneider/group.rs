fn main() {
    println!("{}", group_letter(&Group::AvzRun));
    println!("{}", group_letter(&Group::SpaceGame));
    println!("{}", group_letter(&Group::Plantex));
}
enum Group {
    Plantex,
    AvzRun,
    SpaceGame,
}

fn group_letter(name: &Group) -> char {

    match *name {
        Group::Plantex => 'c',
        Group::SpaceGame => 'b',
        Group::AvzRun => 'a',
    }
}
