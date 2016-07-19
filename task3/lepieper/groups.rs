fn main() {
    println!("{}", group_letter(Groups::Plantex));
    println!("{}", group_letter(Groups::AvzRun));
    println!("{}", group_letter(Groups::SpaceGame));
}
enum Groups {
    Plantex,
    AvzRun,
    SpaceGame,
}
fn group_letter(s: Groups) -> char {
    let k = match s {
        Groups::Plantex => 'c',
        Groups::AvzRun => 'a',
        Groups::SpaceGame => 'b',
    };
    k
}
