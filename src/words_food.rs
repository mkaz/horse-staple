
pub fn words() -> Vec<&'static str> {

	let w = "almond
	apples
	apricot
	avocado
	bacon
	bagel
	barley
	beans
	beef
	beer
	berry
	bread
	cabbage
	cafe
	cake
	carrot
	cheese
	chips
	clams
	coffee
	corn
	cookies
	crab
	cupcakes
	cereal
	dates
	duck
	donuts
	eggs
	eel
	fish
	garlic
	ginger
	goose
	granola
	graphes
	grits
	gumbo
	ham
	halibut
	honey
	hummus
	jam
	jelly
	jerky
	juice
	kale
	kiwi
	lemon
	lime
	lobster
	milk
	noodles
	pizza
	pancakes
	spinach
	toast
	waffles
	wine
	yogurt";

	return w.lines().collect();
}