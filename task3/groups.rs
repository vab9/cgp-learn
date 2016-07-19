fn group_letter(name: &str) -> &str {

	match name {
		"Plantex" => "C",
		"AVZ-Run" => "A",
		"Space Game" => "B",
		_ => "None",
	}
}

fn main() {

	println!("AVZ-Run ist Gruppe {}.", group_letter("AVZ-Run"));
	println!("Space Game ist Gruppe {}.", group_letter("Space Game"));
	println!("Plantex ist Gruppe {}.", group_letter("Plantex"));

}

