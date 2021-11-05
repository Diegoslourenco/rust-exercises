extern crate rand;

use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..101);

    loop {
        println!("Please input your guess.");

        // mut means that the variable is mutable, the default is immutable
        // Instantiating a new string
        // :: means that new is a associated function of the String type
        // In resume, created a mutable variable that is currently bound to a new, empty instance of a String
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)  // receives wherever comes in the input and append it to the string guess
            .expect("Failed to read line");    // handling possible error, the string message will prompt in case of error

        /*
            converts string to 32-bit unsigned int
                if it is possible to convert, return the int
                otherwise, prompt an error
         */
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        // compare the number in guess to the secret_number
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

