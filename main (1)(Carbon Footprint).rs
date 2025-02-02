use std::io;

fn main() {
    // A list of devices with their average power consumption in watts
    let devices = vec![
        ("Phone", 5.0),
        ("Laptop", 60.0),
        ("PlayStation", 150.0),
        ("Nintendo Switch", 18.0),
        ("Xbox", 120.0),
        ("Projector", 250.0),
        ("TV", 100.0),
        ("Desktop Computer", 300.0),
        ("Microwave", 1200.0),
        ("Air Conditioner", 2000.0),
    ];

    println!("Welcome to the Carbon Footprint Calculator!");
    println!("Here's a list of devices. Choose one to calculate its carbon footprint:");

    // Display the devices with their indices
    for (i, device) in devices.iter().enumerate() {
        println!("{}. {}", i + 1, device.0);
    }

    // Prompt the user for their choice
    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Failed to read input");
    
    let choice: usize = match choice.trim().parse::<usize>() {
        Ok(num) if num > 0 && num <= devices.len() => num - 1,
        _ => {
            println!("Invalid choice. Please run the program again and enter a valid number.");
            return;
        }
    };

    // Get the selected device's name and power consumption
    let (device_name, power) = &devices[choice];
    println!("You selected: {}", device_name);

    // Ask for the number of hours the device is used per day
    println!(
        "How many hours per day do you use your {}? (Enter a number)",
        device_name
    );

    let mut hours = String::new();
    io::stdin().read_line(&mut hours).expect("Failed to read input");
    let hours: f64 = match hours.trim().parse::<f64>() {
        Ok(num) if num >= 0.0 => num,
        _ => {
            println!("Invalid input for hours. Please run the program again and enter a valid number.");
            return;
        }
    };

    // Constants
    let emission_factor = 0.4; // kg of CO2 per kWh
    let days_per_year = 365.0; // Assuming daily usage

    // Calculations
    let daily_energy_kwh = (*power / 1000.0) * hours; // Convert watts to kilowatt-hours
    let annual_emissions = daily_energy_kwh * days_per_year * emission_factor;

    // Display the results
    println!(
        "The annual carbon footprint of using your {} for {:.2} hours per day is {:.2} kg CO2e.",
        device_name, hours, annual_emissions
    );

    println!("Thank you for using the Carbon Footprint Calculator!");
}
