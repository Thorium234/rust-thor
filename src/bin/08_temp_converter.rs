use std::io;

fn main() {
    println!("Temperature Converter");
    println!("====================");
    
    loop {
        println!("
Select an option:");
        println!("1. Celsius to Fahrenheit");
        println!("2. Fahrenheit to Celsius");
        println!("3. Quit");
        
        let choice = get_user_input("Enter your choice (1-3): ");
        
        match choice.trim() {
            "1" => celsius_to_fahrenheit(),
            "2" => fahrenheit_to_celsius(),
            "3" => {
                println!("Goodbye!");
                break;
            }
            _ => println!("Invalid choice, please try again."),
        }
    }
}

fn get_user_input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    input
}

fn celsius_to_fahrenheit() {
    let input = get_user_input("Enter temperature in Celsius: ");
    
    // Parse the input as a floating-point number
    match input.trim().parse::<f64>() {
        Ok(celsius) => {
            let fahrenheit = (celsius * 9.0 / 5.0) + 32.0;
            println!("{:.2}°C is {:.2}°F", celsius, fahrenheit);
        }
        Err(_) => {
            println!("Invalid input. Please enter a number.");
        }
    }
}

fn fahrenheit_to_celsius() {
    let input = get_user_input("Enter temperature in Fahrenheit: ");
    
    // Parse the input as a floating-point number
    match input.trim().parse::<f64>() {
        Ok(fahrenheit) => {
            let celsius = (fahrenheit - 32.0) * 5.0 / 9.0;
            println!("{:.2}°F is {:.2}°C", fahrenheit, celsius);
        }
        Err(_) => {
            println!("Invalid input. Please enter a number.");
        }
    }
}
