fn main() {
    println!("{:?}", timely_greeting_match(4));
    println!("{:?}", timely_greeting_match(8));
    println!("{:?}", timely_greeting_match(18));
    println!("{:?}", timely_greeting_match(23));

}

// /// Alte Variante mit if
// fn timely_greeting_if(x: u8) -> &'static str {
//     // static Vorschlag von Compiler
//     if x >= 8 && x <= 12 {
//         "Guten Morgen"
//     } else if x >= 18 && x <= 22 {
//         "Guten Abend"
//     } else {
//         "Hallo"
//     }
// }

/// Variante mit match
fn timely_greeting_match(x: u8) -> String {
    match x {
        // 8...12 => "Guten Morgen"::to_string(&self), //Frage: Wie genau würde das gehen?
        8...12 => "Guten Morgen".to_string(),
        18...22 => "Guten Abend".to_string(),
        0...5 => format!("Warum bist du denn um {X} Uhr noch wach?", X = x),
        _ => "Hallo".to_string(),//deault
    }
}
