fn main() {
    println!("{:?}", timelygreetingif(0));
    println!("{:?}", timelygreetingmatch(8));
    println!("{:?}", timelygreetingif(12));
    println!("{:?}", timelygreetingmatch(18));
    println!("{:?}", timelygreetingif(22));
    println!("{:?}", timelygreetingmatch(23));

}

// fn timelygreeting(x: u8) {
//     let time = if x >= 8 && x <= 12 {
//         "morning"
//     } else if x >= 18 && x <= 22 {
//         "evening"
//     } else {
//         "allday"
//     };

//     let greeting = match time {
//         morning => "Guten Morgen",
//         evening => "Guten Abend",
//         allday => "Hallo",
//     };

//     println!("{}", greeting);
// }

/// Variante mit if
fn timely_greeting_if(x: u8) -> &'static str {
    // static Vorschlag von Compiler
    if x >= 8 && x <= 12 {
        "Guten Morgen"
    } else if x >= 18 && x <= 22 {
        "Guten Abend"
    } else {
        "Hallo"
    }
}

/// Variante mit match
fn timely_greeting_match(x: u8) -> &'static str {
    match x {
        8...12 => "Guten Morgen",
        18...22 => "Guten Abend",
        _ => "Hallo",//deault
    }
}
