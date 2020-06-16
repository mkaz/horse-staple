pub fn words() -> Vec<&'static str> {

	let w = "one
	two
	three
	four
	five
	six
	seven
	eight
	nine
	ten
	eleven
	twelve
	thirteen
	fourteen
	fifteen
	sixteen
	seventeen
	eighteen
	ninety
	twenty";

	return w.lines().collect();
}