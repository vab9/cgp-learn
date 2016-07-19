fn main(){
    println!("{}", group_letter("Plantex"));
}

fn group_letter(name: &str) -> char{

       match name{
            "AVZ-Run" => 'A',
            "Space Game"  => 'B',
            "Plantex" => 'C',
            _ => 'X',
       }


}
