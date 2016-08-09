use std::io;

fn main() {
	println!("Please choose a number.");

	let mut number = String::new();
	
	io::stdin().read_line(&mut number)
		.ok()
		.expect("failed to read line");

	let mut number : u64 = number.trim().parse()
		.ok()
		.expect("failed to convert to number");

	println!("You chose: {}", number);

	let mut n = 1;
	loop {
		println!("Iteration	{}, number: {}", n, number);
		if (number == 1) {
			break;
		}
		if (number % 2 == 1) {
			number = 3 * number + 1;
		}
		else {
			number = number / 2;
		}
		n += 1;
	}
}
