fn main(){
    println!("{}", group_letter(Game::Plantex));
    println!("{}", group_letter(Game::SpaceGame));
    println!("{}", group_letter(Game::AVZRun));
}

fn group_letter(name: Game) -> char{

       match name{
            Game::AVZRun => 'A',
            Game::SpaceGame  => 'B',
            Game::Plantex => 'C',
       }
}

enum Game{AVZRun, SpaceGame, Plantex}
