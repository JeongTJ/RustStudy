use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Hello, world!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

	loop {
		let mut str = String::new();
		io::stdin().read_line(&mut str).expect("Failed to read line");

		let str: u32 = match str.trim().parse() {
			Ok(num) => num,
			Err(_) => {
				println!("Please number");
				continue ;
			},
		};

		println!("Your number is {str}");
		match str.cmp(&secret_number) {
			Ordering::Less => println!("Too small"),
			Ordering::Equal => {
				println!("Equal");
				break ;
			},
			Ordering::Greater => println!("Too big"),
		}
	}
}
