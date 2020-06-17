/*
 * Password Generator
 */
use clap::{Arg,App};
use rand::seq::SliceRandom;
use num_format::{Locale, ToFormattedString};

mod words_animals;
mod words_basic;
mod words_colors;
mod words_food;
mod words_house;
mod words_misc;
mod words_travel;

fn main() {
	let matches = App::new("horse-staple")
		.version("0.2.0")
		.about("Generates memorable passwords")
		.author("Marcus Kazmierczak")
		.arg(Arg::new("length")
			.about("Number of words")
			.short('l')
			.long("length")
			.takes_value(true),
		)
		.get_matches();

	let mut pass: Vec<&str> = Vec::new();
	let mut words: Vec<&str> = Vec::new();

	let length: u32 = matches.value_of_t("length").unwrap_or(4);

	// load vector
	words.append(&mut words_animals::words());
	words.append(&mut words_basic::words());
	words.append(&mut words_colors::words());
	words.append(&mut words_food::words());
	words.append(&mut words_house::words());
	words.append(&mut words_misc::words());
	words.append(&mut words_travel::words());

	words.sort();
	words.dedup();
	let size = words.len();
	let combo = size.pow(length);
	println!("Dictionary  : {} words", size.to_formatted_string(&Locale::en));
	println!("Combinations: {}", combo.to_formatted_string(&Locale::en));
	println!("");

	// pick random word
	while pass.len() < (length as usize) {
		let word = words.choose(&mut rand::thread_rng()).unwrap();
		let trimmed = word.trim();
		if ! pass.contains(&trimmed) {
			pass.push(&trimmed);
		}
	}


	// output password
	for p in pass {
		print!("{} ", p);
	}
	println!("\n");
}

