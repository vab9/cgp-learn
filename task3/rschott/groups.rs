
enum Group {
    Plantex,
    AVZRun,
    SpaceGame,
}
fn main() {
    println!("Plantex is group {}", group_letter(Group::Plantex));
    println!("AVZ-Run is group {}", group_letter(Group::AVZRun));
    println!("Space Game is group {}", group_letter(Group::SpaceGame));


}

fn group_letter(c: Group) -> char {
    match c {
        Group::Plantex => 'c',
        Group::AVZRun => 'a',
        Group::SpaceGame => 'b',
    }

}
