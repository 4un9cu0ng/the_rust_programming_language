fn main() {
	let fah = 212;
	println!("Convert {:?} fahrenhiet to celsius: {:?}", fah, convert_fahrenheit_to_celsius(fah));

	let cel = 100;
	println!("Convert {:?} celsius to fahrenhiet: {:?}", cel, convert_celsius_to_fahrenheit(cel));
}

fn convert_fahrenheit_to_celsius (fah: u32) -> u32 {
	(fah - 32) * 5 / 9
}

fn convert_celsius_to_fahrenheit (cel: u32) -> u32 {
	(cel * 9 / 5) + 32
}
