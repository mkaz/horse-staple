/*
 * Password Generator
 *
 */
use rand::seq::SliceRandom;

mod words_colors;
mod words_food;
mod words_locations;
mod words_misc;

fn main() {
	println!("Horse Staple");
	let mut pass: Vec<&str> = Vec::new();
	let mut words: Vec<&str> = Vec::new();

	let length = 3;

	// load vector
	words.append(&mut words_colors::words());
	words.append(&mut words_food::words());
	words.append(&mut words_locations::words());
	words.append(&mut words_misc::words());

	println!("Dictionary: {} words", words.len());

	// pick random word
	for _ in 0..length {
		let word = words.choose(&mut rand::thread_rng()).unwrap();
		pass.push( word.trim() );
	}

	// output three
	for p in pass {
		print!("{} ", p);
	}
	print!("\n");
}

