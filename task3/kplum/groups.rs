fn main() {
    println!("{}", group_letter(&GroupName::Plantex));
    println!("{}", group_letter(&GroupName::AvzRun));
    println!("{}", group_letter(&GroupName::SpaceGame));
}


enum GroupName {
    Plantex,
    AvzRun,
    SpaceGame,
}

fn group_letter(name: &GroupName) -> char {
    match *name {
        GroupName::Plantex => 'C',
        GroupName::AvzRun => 'A',
        GroupName::SpaceGame => 'B',
    }
}
