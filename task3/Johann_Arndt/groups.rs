enum Groups {
    Plantex,
    AvzRun,
    SpaceGame,
}

fn main() {
    println!("{}", group_letter(Groups::Plantex));
}

fn group_letter(name: Groups) -> char {
    match name {
        Groups::Plantex => 'C',
        Groups::AvzRun => 'B',
        Groups::SpaceGame => 'A',
    }
}
