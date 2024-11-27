// Rust program to count numbers

use std::io;

fn main(){

	let mut input1 = String::new();
	let mut input2 = String::new();

	println!("Enter lower bound: ");
	io::stdin().read_line(&mut input1).expect("Failed to read input");
	let lower_bound:u32 = input1.trim().parse().expect("Failed to input");

	println!("Enter upper bound");
	io::stdin().read_line(&mut input2).expect("Failed to read input");
	let upper_bound:u32 = input2.trim().parse().expect("Failed to input");

	for x in lower_bound..upper_bound{ // lower_bound is not included

		println!("Count Level is {}",x);
   }
}
