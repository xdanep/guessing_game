// Libraries
use std::io; // io from std library
use rand::Rng;  // rand library
use std::cmp::Ordering; 

// Main function
fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100); // generate a number between 1 and 100

    loop{
        println!("Please input your guess!:");
        let mut guess = String::new();          // mutable variable, type string, ::new() creates a new empty string

        io::stdin()                               // receiving user input
            .read_line(&mut guess) // get input from user
            .expect("Failed to read line");        // error handling

        let guess: u32 = match guess.trim().parse() {   // match is like switch
            Ok(num) => num,
            Err(_) => continue,
        };                                              // trim eliminates \n, parse to convert string to another type
        
        println!("You guessed {guess}");                // between {} for printing variables

        match guess.cmp(&secret_number) {               // compare values
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win");
                break;
            }
        }
    }
}
