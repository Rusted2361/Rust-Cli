use std::io;
use rand::Rng;
fn main() {
    let mut input = String::new();
    // Prompt the user for input
     println!("Enter a number between 1 and 100:");

    // Read the input
    io::stdin().read_line(&mut input).unwrap();

    // Parse the input as a number
    let number: u32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input. Please enter a number.");
            return;
        }
    };

    //generate random number
    let mut rng = rand::thread_rng();
    let rand_num: u32 = rng.gen_range(0..100);
    println!("Random number: {}", rand_num);

    // Check if the number is within the valid range
    if number == rand_num {
        println!("You entered {} and it matched", number);
    } else if number > rand_num {
        println!("number is too big than random number");
    } else if number < rand_num {
        println!("number is too small than random number");
    }
    println!("{}", input);
}
