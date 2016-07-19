#[derive(Debug)]
enum Group {
    Plantex,
    AVZRun,
    SpaceGame,
}

fn main() {
    let mut passed = true;

    if !(group_letter(&Group::Plantex) == 'c') {
        passed = false;
    };
    if !(group_letter(&Group::AVZRun) == 'a') {
        passed = false;
    };
    if !(group_letter(&Group::SpaceGame) == 'b') {
        passed = false;
    };

    println!("{}",
             if passed {
                 "passed"
             } else {
                 "failed"
             });
}

fn group_letter(a: &Group) -> char {
    match a {
        &Group::Plantex => 'c',
        &Group::AVZRun => 'a',
        &Group::SpaceGame => 'b',
    }
}
