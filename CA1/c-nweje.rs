// Rust program to calculate the charges for patients health diagnosis

use std::io;

fn main(){

	let mut input1 = String::new();
	let mut input2 = String::new();
	let mut input3 = String::new();
	let mut input4 = String::new();
	let mut input5 = String::new();
	let mut input6 = String::new();
	let mut input7 = String::new();
	let mut input8 = String::new();

	println!("Enter name: ");
	io::stdin().read_line(&mut input1).expect("Failed to read input");

	println!("Enter date of birth: ");
	io::stdin().read_line(&mut input2).expect("Failed to read input");
	let date of birth:u32 = input2.trim().parse().expect("Failed to parse input");

	println!("Enter email address: ");
	io::stdin().read_line(&mut input3).expect("Failed to read input");

	println!("Enter phone number: ");
	io::stdin().read_line(&mut input4).expect("Failed to read input");
	let phone number:u32 = input4.trim().parse().expect("Failed to parse input");

	println!("Enter number of siblings: ");
	io::stdin().read_line(&mut input5).expect("Failed to read input");
	let number of siblings:u32 = input5.trim().parse().expect("Failed to parse input");

	println!("Enter number of children: ");
	io::stdin().read_line(&mut input6).expect("Failed to read input");
	let number of children:u32 = input6.trim().parse().expect("Failed to parse input");

	println!("Enter medical diagnosis: ");
	io::stdin().read_line(&mut input7).expect("Failed to read input");

	println!("Enter village of residence: ");
	io::stdin().read_line(&mut input8).expect("Failed to read input");

	if medical diagnosis is alzheimer && date of birth < 1974 && number of children > 4 && village of residence is Akpabom {
	   println!("Amount is 240,000");
	} else {
	   println!("Amount is 1,200,000");
	}  
	if medical diagnosis is Arrythmia && date of birth = 1994 && number of children > 4 && village of residence is Ngbauji{
        println!("Amount is 27,500");
    } else  {
    	println!("Amount is 550,000");
    }
    if medical diagnosis is Chronic Kidney disease && date of birth < 1984 && number of children > 3 && number of siblings > 3 && village of residence is Atabrikang{
        println!("Amount is 225,000");
    }  else  {
        println!("Amount is 1,500,000");
    }
    if medical diagnosis is diabetes && date of birth < 1996 && date of birth > 1979 && number of children = 2 to 4 && village of residence is Okorobilom{
    	println!("Amount is 80,000");
    }  else {
    	println!("Amount is 800,000");
    }
    if  medical diagnosis is Arthritis && birth < 1966 & number of siblings > 5 && number of children > 5 && village of residence is Emeremen{
    	println!("Amount is 45,000");
    }  else {
    	println!("Amount is 450,000");
    }
 //All conditions apply to the first 100 patients that visit the clinic each day
 }


