
enum Group {
    Plantex,
    AvzRun,
    SpaceGame,
}
fn main() {
    println!("Plantex is group {}", group_letter(Group::Plantex));
    println!("AVZ-Run is group {}", group_letter(Group::AvzRun));
    println!("Space Game is group {}", group_letter(Group::SpaceGame));


}

fn group_letter(c: Group) -> char {
    match c {
        Group::Plantex => 'c',
        Group::AvzRun => 'a',
        Group::SpaceGame => 'b',
    }

}
