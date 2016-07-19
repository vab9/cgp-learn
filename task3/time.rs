fn greet(time: u8) -> String {
	
	match time {
		0...7 => "Hallo".to_string(),
		8...12 => "Guten Morgen".to_string(),
		13...17 => "Hallo".to_string(),
		18...22 => "Guten Abend".to_string(),
		22...24 => "Hallo".to_string(),
		_ => "So eine Uhrzeit haben wir nicht.".to_string(),
	}
}

fn main() {
	
	println!("Um 11.00 Uhr sagen wir {}.", greet(11));
	println!("Um 23.00 Uhr sagen wir {}.", greet(23));
	println!("Um 19.00 Uhr sagen wir {}.", greet(19));
	println!("Bei 30: {}", greet(30));

}

