fn main() {
    let y = {
    	let x = 3;
    	x  + 1
    };

    println!("the value of y is: {:?}", y);

    define_fn();

    let x = five();
    println!("the value of x is: {:?}", x);

    let y = plus_one(1000);
    println!("The value of y is: {}", y);

    let number = 3;

    if number < 5 {
    	println!("condition was true");
    } else {
    	println!("condition was false");
    }

    let condition = true;
    let number = if condition {
    	5
    } else {
    	6
    };

    println!("The value of number is: {}", number);

    // try_loop();

    try_while();

    try_while_with_array();

    try_for();

    try_for_other();
}

fn define_fn () {
	println!("Test something ");
}

fn five() -> i32 {
	5
}

fn plus_one(x: i32) -> i32 {
	x + 1
}

fn try_loop() {
	let mut i = 0;
	loop {
		i = i + 1;
		println!("{} again!", i);
	}
}

fn try_while() {
	let mut number = 3;
	while number != 0 {
		println!("{:?}!", number);
		number = number - 1;
	}

	println!("LIFTOFF!!!\n");
}

fn try_while_with_array() {
	let a = [10, 20, 30, 40, 50];
	let mut index = 0;

	while index < 5 {
		println!("the value is: {}", a[index]);
		index = index + 1;
	}
	println!("LIFTOFF!!!\n");
}

fn try_for() {
	let a = [10, 20, 30, 40, 50];

	for element in a.iter() {
		println!("the value is: {}", element);
	}
	println!("LIFTOFF!!!\n");
}

fn try_for_other() {
	for number in (1..4).rev() {
		println!("{:?}", number);
	}
	println!("LIFTOFF!!!\n");
}