use std::io;

fn main() {
    println!("Hello, Pick one:");
    println!("1. Celsius to Fahrenheit");
    println!("2. Fahrenheit to Celsius");

    let choices: [i8; 2] = [1, 2];
    let mut _choice: i8 = 0;
    let mut input = String::new();
    
    println!("Enter your choice 1 or 2:");
    io::stdin().read_line(&mut input).expect("Failed to read the line");
    
    match input.trim().parse::<i8>() {
        Ok(num) => {
            if choices.contains(&num) {
                println!("You've chosen: {}", num);
                _choice = num;
            } else {
                println!("Pick between 1 or 2");
                return;
            }
        }
        Err(_) => {
            println!("That's not a valid number!");
            return;
        }
    }
    
    // Clear the input string before reading the next input
    input.clear();
    
    let (_celcius, _farenheit): (f32, f32);
    
    if _choice == 1 {
        println!("Enter temperature in Celsius:");
        io::stdin().read_line(&mut input).expect("Failed to read line");
        
        _celcius = match input.trim().parse::<f32>() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid floating-point number!");
                return;
            }
        };
        
        _farenheit = celcius_farenheit(_celcius);
        println!("Temperature in Fahrenheit: {:.2}", _farenheit);
        
    } else if _choice == 2 {
        println!("Enter temperature in Fahrenheit:");
        io::stdin().read_line(&mut input).expect("Failed to read line");
        
        _farenheit = match input.trim().parse::<f32>() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid floating-point number!");
                return;
            }
        };
        
        _celcius = farenheit_celcius(_farenheit);
        println!("Temperature in Celsius: {:.2}", _celcius);
    }
}

fn celcius_farenheit(c: f32) -> f32 {
    (c * 1.8) + 32.0
}

fn farenheit_celcius(f: f32) -> f32 {
    (f - 32.0) / 1.8
}
