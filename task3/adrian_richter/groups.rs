#[derive(Debug)]
enum Group {
    Plantex,
    AvzRun,
    SpaceGame,
}

fn main() {
    assert_eq!(group_letter(&Group::Plantex), 'c');
    assert_eq!(group_letter(&Group::AvzRun), 'a');
    assert_eq!(group_letter(&Group::SpaceGame), 'b');

    println!("klappt");
}

fn group_letter(a: &Group) -> char {
    match a {
        &Group::Plantex => 'c',
        &Group::AvzRun => 'a',
        &Group::SpaceGame => 'b',
    }
}
