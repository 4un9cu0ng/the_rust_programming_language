fn main() {
	// let s = String::from("Hello");
	let mut s = String::from("Hello");
	s.push_str(", world!");
	println!("{}", s);

	let s1 = String::from("hello");
	let s2 = s1;

	println!("{}", s1);
}