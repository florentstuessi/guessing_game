use std::io;

use rand::Rng;

fn main() {
    println!("Guess the number!");

    // thread_rng() is a function that provides a random number generator
    // gen_range(1..=100) generates a random number in the range 1 to 100
    // The `=` in the range means that 100 is inclusive
    let secret_number = rand::thread_rng()
                                .gen_range(1..=100);

    println!("The secret number is: {secret_number}");

    println!("Please input your guess.");
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}
