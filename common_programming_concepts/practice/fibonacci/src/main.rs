fn main() {
    fibonacci(10);
}

fn fibonacci(n: i32) {
	let mut x = 0;
	let mut y = 1;
	for _ in (0..n).rev() {
		let z = x + y;
		y = x;
		x = z;
		print!("{:?} ", z);
	}
}
