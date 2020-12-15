use std::io;

fn main() {
    loop {
        let mut input = String::new();

        println!("Type (1) or (2) and hit [Enter] to choose:\n1. Convert Fahrenheit to Celsius\n2. Convert Celsius to Fahrenheit");

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let input: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if input == 1 {
            let temp = get_temp();
            println!("{:.1}°C is {:.1}°F", temp, f_to_c(&temp));
            break;
        } else if input == 2 {
            let temp = get_temp();
            println!("{:.1}°C is {:.1}°F", temp, c_to_f(&temp));
            break;
        } else {
            continue;
        }
    }

}

fn get_temp() -> f64 {
    let mut input = String::new();
    loop {
        println!("Enter the temperature");

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let input: f64 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        return input;
    }
}

// Converts Fahrenheit to Celsius
fn f_to_c(temp: &f64) -> f64 {
    (temp-32.)*(5./9.)
}

// Converts Celsius to Fahrenheit
fn c_to_f(temp: &f64) -> f64 {
    temp*(9./5.)+32.
}