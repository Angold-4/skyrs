use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    // thread_rng gives the particular random number generator that we are going to use

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin() // returns an instance of std::io::Stdin
            .read_line(&mut guess) // passing by reference
            // readline puts whatever the user enters into the string guess
            // and then return a Result->enumeration which is a type that can
            // be in one of multiple possible states -> variantes (Err / Ok)
            .expect("Failed to read line"); // expect is one of the method of type Result

        // trim() removes the \n / whitespace around guess
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
            // "_" catchall value
        };

        println!("You guessed: {guess}"); // {} placeholder

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
