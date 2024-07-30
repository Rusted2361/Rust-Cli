use std::io;

fn main() {
    println!("please select 0 for celsius-fahrenheit and 1 for fahrenheit-celsius");
    let number = read_input_line_f32();
    // Check if the number is within the valid range
    if number == 0.0 {
        println!("Enter the temperature in celsius");
        let celcius_input: f32 = read_input_line_f32();
        let fahrenheit: f32 = celcius_to_fahrenheit(celcius_input);
        println!(
            "{} degrees celcius is {} fahrenheit.",
            celcius_input, fahrenheit
        );
    } else if number == 1.0 {
        println!("Enter the temperature in fahrenheit");
        let fahrenheit_input: f32 = read_input_line_f32();
        let celcius: f32 = fahrenheit_to_celcius(fahrenheit_input);
        println!(
            "{} degrees fahrenheit is {} celcius.",
            fahrenheit_input, celcius
        );
    }

    fn read_input_line_f32() -> f32 {
        loop {
            let mut input_string = String::new();

            io::stdin()
                .read_line(&mut input_string)
                .expect("Failed to read line");

            let input_string: f32 = match input_string.trim().parse() {
                Ok(num) => return num,
                Err(_) => continue,
            };
        }
    }

    fn celcius_to_fahrenheit(celcius: f32) -> f32 {
        let fahrenheit = (celcius * 1.8) + 32.0;
        return fahrenheit;
    }

    fn fahrenheit_to_celcius(fahrenheit: f32) -> f32 {
        let celcius = (fahrenheit - 32.0) / 1.8;
        return celcius;
    }
}
