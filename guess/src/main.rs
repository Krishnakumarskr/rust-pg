use std::io;
use rand::Rng;
use std::cmp::Ordering;
use colored::*;

fn main() {
    
    println!("Welcome to the guessing game!...");

    let secret_number = rand::rng().random_range(1..101);

    println!("The secret number is: {}", secret_number);

    loop {

        println!("Please enter your number");

        let mut guess = String::new();


        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line!");

        let guessed_number: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter correct number!");
                continue;
            }
        };

        println!("The guessed number: {}", guessed_number);

        match guessed_number.cmp(&secret_number) {
            Ordering::Equal => {
                println!("{}", "Congrats! You guessd the number!".green());
                break;
            },
            Ordering::Greater => println!("{}", "It's greater than the secret!".red()),
            Ordering::Less => println!("{}", "It's less than the secret!".red())
        }
    }
}
