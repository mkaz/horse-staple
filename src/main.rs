/*
 * Password Generator
 *
 */
use rand::seq::SliceRandom;
use num_format::{Locale, ToFormattedString};

mod words_colors;
mod words_food;
mod words_locations;
mod words_misc;
mod words_site;

// TODO: de-dupe password
// --no-locations
// --length
// --lowercase
// --digit separator

fn main() {
	println!("---------------------------");
	println!("Horse Staple Password Maker");
	println!("---------------------------");
	let mut pass: Vec<&str> = Vec::new();
	let mut words: Vec<&str> = Vec::new();

	let length = 4;

	// load vector
	words.append(&mut words_colors::words());
	words.append(&mut words_food::words());
	words.append(&mut words_locations::words());
	words.append(&mut words_misc::words());
	words.append(&mut words_site::words());

	words.sort();
	words.dedup();
	let size = words.len();
	let combo = size*size*size*size;
	println!("Dictionary: {} words", size);
	println!("Combinations: {}", combo.to_formatted_string(&Locale::en));
	println!("");

	// pick random word
	while pass.len() < length {
		let word = words.choose(&mut rand::thread_rng()).unwrap();
		let trimmed = word.trim();
		if ! pass.contains(&trimmed) {
			pass.push(&trimmed);
		}
	}

	// output three
	for p in pass {
		print!("{} ", p);
	}
	println!("\n");
}

