enum GroupName {
    Plantex,
    AVZ_Run,
    Space_Game,
}

fn main() {
    println!("Plantex: {}", group_letter(GroupName::Plantex));
    println!("AVZ_Run: {}", group_letter(GroupName::AVZ_Run));
    println!("Space_Game: {}", group_letter(GroupName::Space_Game));
}

fn group_letter(name: GroupName) -> char {
    match name {
        GroupName::Plantex => 'c',
        GroupName::AVZ_Run => 'a',
        GroupName::Space_Game => 'b',
    }
}
