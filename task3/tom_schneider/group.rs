fn main() {
    println!("{}", group_letter(&Group::AVZRun));
    println!("{}", group_letter(&Group::SpaceGame));
    println!("{}", group_letter(&Group::Plantex));
}
enum Group {
    Plantex,
    AVZRun,
    SpaceGame,
}

fn group_letter(name: &Group) -> char {

    match *name {
        Group::Plantex => 'c',
        Group::SpaceGame => 'b',
        Group::AVZRun => 'a',
    }
}
