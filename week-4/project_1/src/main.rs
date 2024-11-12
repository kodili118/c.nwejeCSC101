use std::io;

fn main() {
	let mut input1 = String::new();
	let mut input2 = String::new();
	let mut input3 = String::new();

	println!("Enter a: ");
	io::stdin().read_line(&mut input1).expect("Failed to read input");
	let a: f32 = input1.trim().parse().expect("Failed to parse input");

	println!("Enter b: ");
	io::stdin().read_line(&mut input2).expect("Failed to read input");
	let b: f32 = input2.trim().parse().expect("Failed to parse input");

	println!("Enter c: ");
	io::stdin().read_line(&mut input3).expect("Failed to read input");
	let c: f32 = input3.trim().parse().expect("Failed to parse input");

	let discriminant = b * b - 4.0 * a * c;
	println!("Discriminant is {}", discriminant);

	if discriminant > 0.0 {
		let x1 = (-b + discriminant.sqrt()) / (2.0 * a);
		let x2 = (-b - discriminant.sqrt()) / (2.0 * a);
		println!("The square roots are {} and {}", x1, x2);
    } else if discriminant == 0.0 {
    	let x = -b / (2.0 * a);
    	println!("The root is {}", x);
    } else {
    	println!("The equation has no real roots");
    }
}
 