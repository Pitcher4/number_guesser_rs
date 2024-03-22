// Crates
use rand::Rng;
use std::io;

// Constants (things which will not be changed outside the code)
const NUM_GUESSES: u8 = 10;
const LOWER_BOUND: u8 = 0;
const UPPER_BOUND: u8 = 100;

// Main function (main code)
fn main() {
    // Defining variable
    let target: u8 = rand::thread_rng().gen_range(LOWER_BOUND..UPPER_BOUND);
    println!("Welcome to the Number Guessing Game.");

    // Repeat NUM_GUESSES times (or repeat until correct guess)
    for _i in 0..NUM_GUESSES {
        println!("Please enter your guess: ");

        // Getting input from user
        let mut input: String = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Invalid input. Please try again.");

        // Parses the input into a u8 integer
        let guess: u8 = input
            .trim()
            .parse()
            .expect("Invalid input. Please enter an integer, not a string.");

        // Handles the guess
        if target == guess {
            println!("You guessed the correct number!");
            break;
        } else if guess > target {
            println!("Incorrect. Try a smaller number");
        } else if guess < target {
            println!("Incorrect. Try a bigger number");
        }
    }
}
