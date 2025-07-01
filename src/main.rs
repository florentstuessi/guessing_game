use std::io;
use std::cmp::Ordering;

use rand::Rng;

fn main() {
    println!("Guess the number!");

    // thread_rng() is a function that provides a random number generator
    // gen_range(1..=100) generates a random number in the range 1 to 100
    // The `=` in the range means that 100 is inclusive
    let secret_number = rand::rng()
                                .random_range(1..=100);

    println!("The secret number is: {secret_number}");

    println!("Please input your guess.");

    loop {
        let mut guess = String::new();
        // Read user input
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        // Trim the input to remove whitespace and parse it to a number
        // expect() is used to handle the case where parsing fails
        // It will panic with the message "Please type a number!" if parsing fails
        // The `trim()` method removes any leading or trailing whitespace
        // The `parse()` method converts the string to a number
        let guess: u32 = guess.trim().parse().expect("Please type a number!");
        println!("You guessed: {guess}");
        // Match the guess against the secret number
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }   
    }
}
