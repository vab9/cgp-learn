enum GroupName {
    Plantex,
    AvzRun,
    SpaceGame,
}

fn main() {
    println!("Plantex: {}", group_letter(GroupName::Plantex));
    println!("AvzRun: {}", group_letter(GroupName::AvzRun));
    println!("SpaceGame: {}", group_letter(GroupName::SpaceGame));
}

fn group_letter(name: GroupName) -> char {
    match name {
        GroupName::Plantex => 'c',
        GroupName::AvzRun => 'a',
        GroupName::SpaceGame => 'b',
    }
}
