fn main() {
    println!("{}", group_letter(&GroupName::Plantex));
    println!("{}", group_letter(&GroupName::AVZRun));
    println!("{}", group_letter(&GroupName::SpaceGame));
}


enum GroupName {
    Plantex,
    AVZRun,
    SpaceGame,
}

fn group_letter(name: &GroupName) -> char {
    match *name {
        GroupName::Plantex => 'C',
        GroupName::AVZRun => 'A',
        GroupName::SpaceGame => 'B',
    }
}
