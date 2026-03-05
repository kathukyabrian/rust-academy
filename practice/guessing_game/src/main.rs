/**
Computer generates a random number

User guesses

Program tells them:
    too high
    too low
    correct
**/
use std::cmp::Ordering;
use std::io;
use rand::Rng;

fn main() {
    println!("Hello, welcome to the guessing game...");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess");


        // create a variable to hold the guess
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read line");

        let guess: u32 = guess.trim().parse()
            .expect("please type a number");
        println!("you guessed {guess}");

        match guess.cmp(&secret_number){
            Ordering::Less => println!("Too Low"),
            Ordering::Greater => println!("Too High"),
            Ordering::Equal => { 
                println!("Equal");
                println!("Quitting....");
                break;
            },
        }
    }
}
