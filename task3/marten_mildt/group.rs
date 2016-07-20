enum Game {
    AvzRun,
    Plantex,
    SpaceShooter,
}

fn main() {
    println!("{}", group_letter(Game::Plantex));
}

fn group_letter(name: Game) -> char {
    match name {
        Game::AvzRun => 'a',
        Game::SpaceShooter => 'b',
        Game::Plantex => 'c',
    }
}
