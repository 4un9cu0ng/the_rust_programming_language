fn main() {
    twelve_days_of_christmas();
}

fn twelve_days_of_christmas() {
	let days = ["first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth", "tenth", "eleventh", "twelfth"];
	// let numbers = ["one", "two", "three", "four", "five", "six", "sevent", "eight", "nine", "ten", "eleven", "twelve"];

	let defines = [
		"A partridge in a pear tree", 
		"Two turtle doves and a partridge in a pear tree", 
		"Four calling birds, three French hens",
		"Three French hens, two turtle doves and a partridge in a pear tree", 
		"Five gold rings, four calling birds, three French hens",
		"Six geese a laying, five gold rings",
		"Seven swans a swimming, six geese a laying, five gold rings",
		"Eight maids a milking, seven swans a swimming",
		"Nine ladies dancing, eight maids a milking",
		"Ten lords a leaping, nine ladies dancing, eight maids a milking",
		"Eleven pipers piping, ten lords a leaping",
		"Twelve drummers drumming, eleven pipers piping"
	];

	println!("Lyrics");

	let mut index = 0;
	for day in days.iter() {
		if index != 8 {
			println!("On the {} day of Christmas my true love gave to me", day);
		} else {
			println!("On the {} day of Christmas", day);
		}

		if [0, 4, 7, 11, 10].contains(&index) {
			println!("{}", defines[index]);
		}

		if index == 2 {
			println!("{}", defines[3]);
		}

		if index == 9 || index == 11 {
			println!("{}", defines[9]);
		}

		if index == 8 || index == 10 {
			println!("{}", defines[8]);
		}

		if index == 5 || index == 7 {
			println!("{}", defines[5]);
		}

		if index > 5 && index != 7 {
			println!("{}", defines[6]);
		}

		if index == 3 || index > 4 {
			println!("{}", defines[2]);
		} 

		if index == 1 || index > 2 {
			println!("{}", defines[1]);
		}

		println!("");
		index = index + 1;
	}
}
