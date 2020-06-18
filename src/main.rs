/*
 * Password Generator
 */
use clap::{Arg,App};
use rand::seq::SliceRandom;
use rand::Rng;
use num_format::{Locale, ToFormattedString};

mod words_adjs;
mod words_nouns;
mod words_verbs;

fn main() {
	let matches = App::new("horse-staple")
		.version("0.2.0")
		.about("Generates memorable passwords")
		.author("Marcus Kazmierczak")
		.args(&[
			Arg::with_name("length")
				.about("Number of words")
				.short('l')
				.long("length")
				.takes_value(true),
			Arg::with_name("digits")
				.about("Include numbers")
				.short('d')
				.long("digits"),
			Arg::with_name("kids")
				.about("Simple adj + noun password")
				.short('k')
				.long("kids"),
		]).get_matches();

	let mut pass: Vec<String> = Vec::new();
	let mut words: Vec<&str> = Vec::new();

	let length: u32 = matches.value_of_t("length").unwrap_or(4);

	if matches.is_present("kids") {

		if matches.is_present("digits") {
			let num:i32 = rand::thread_rng().gen_range(1, 100);
			pass.push(num.to_string());
		}

		// pick random adjective
		let adjs = words_adjs::words();
		let adj = adjs.choose(&mut rand::thread_rng()).unwrap();
		pass.push(adj.trim().to_string());

		let nouns = words_nouns::words();
		let noun = nouns.choose(&mut rand::thread_rng()).unwrap();
		pass.push(noun.trim().to_string());

		let verbs = words_verbs::words();
		let verb = verbs.choose(&mut rand::thread_rng()).unwrap();
		pass.push(verb.trim().to_string());
	}
	else {

		// load vector
		words.append(&mut words_adjs::words());
		words.append(&mut words_nouns::words());
		words.append(&mut words_verbs::words());

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
			if ! pass.contains(&trimmed.to_string()) {
				pass.push(trimmed.to_string());
			}
		}

		if matches.is_present("digits") {
			let num:i32 = rand::thread_rng().gen_range(1, 1000);
			pass.push(num.to_string());
		}
	}

	// output password
	for p in pass {
		print!("{} ", p);
	}
	println!("\n");
}

