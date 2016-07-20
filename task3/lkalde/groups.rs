/// Get Group Letter for each project group

enum Group {
    Plantex,
    AvzRun,
    SpaceGame,
}

fn main() {
    println!("AvzRun: {}", groupletter(Group::AvzRun));
    println!("SpaceGame: {}", groupletter(Group::SpaceGame));
    println!("Plantex: {}", groupletter(Group::Plantex));
}

fn groupletter(group: Group) -> char {
    match group {
        Group::AvzRun => 'a',
        Group::SpaceGame => 'b',
        Group::Plantex => 'c',
    }
}
