
enum Group {
    Plantex,
    AVZ_Run,
    Space_Game,
}
fn main() {
    let g: Group = Group::Plantex;
    println!("Plantex is group {}", group_letter(g));
    let g: Group = Group::AVZ_Run;
    println!("AVZ-Run is group {}", group_letter(g));
    let g: Group = Group::Space_Game;
    println!("Space Game is group {}", group_letter(g));


}

fn group_letter(c: Group) -> char {
    match c {
        Group::Plantex => 'c',
        Group::AVZ_Run => 'a',
        Group::Space_Game => 'b',
    }

}
