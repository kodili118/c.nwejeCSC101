use std::io;
use std::io::Read;

fn main(){
let mut input1 = String::new();

println!("Enter Job");
io::stdin().read_line(&mut input1).expect("Failed to read input");
let job = input1.trim();

if job == "Administratior"{
	let mut file = std::fs::File::open("globalcom_dbase.sql").unwrap();
	let mut contents = String::new();
	file.read_to_string(&mut contents).unwrap();
	println!("{}", contents);
}else if job == "Project Manager"{
	let mut file = std::fs::File::open("project_table.sql").unwrap();
	let mut contents = String::new();
	file.read_to_string(&mut contents).unwrap();
	println!("{}", contents);
}else if job == "Employee"{
	let mut file = std::fs::File::open("staff_table.sql").unwrap();
	let mut contents = String::new();
	file.read_to_string(&mut contents).unwrap();
	println!("{}", contents);
}else if job == "Customer"{
	let mut file = std::fs::File::open("customer_table.sql").unwrap();
	let mut contents = String::new();
	file.read_to_string(&mut contents).unwrap();
	println!("{}", contents);
}else if job == "Vendor"{
	let mut file = std::fs::File::open("dataplan_table.sql").unwrap();
	let mut contents = String::new();
	file.read_to_string(&mut contents).unwrap();
	println!("{}", contents);
}
}
