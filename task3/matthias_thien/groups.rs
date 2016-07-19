enum Names {
    AvzRun,
    SpaceGame,
    Plantex,
}
fn main() {
    println!("{}", group_letter(Names::AvzRun));
    println!("{}", group_letter(Names::SpaceGame));
    println!("{}", group_letter(Names::Plantex));
}
fn group_letter(input: Names) -> char {
    match input {
        Names::AvzRun => 'A',
        Names::SpaceGame => 'B',
        Names::Plantex => 'C',
    }
}
