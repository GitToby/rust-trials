use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guessing Game!");
    println!("--------------");

    let number: i32 = rand::thread_rng().gen_range(-100, 101); // Random # -100 -> 100
    let mut guesses: i32 = 0; // init the guess count value

    loop {
        println!("Guess the number:");
        let mut guess: String = String::new(); // init he guess value as a string, for reading

        io::stdin().read_line(&mut guess) // read the string in, if it fails, we have an error message
            .expect("Something went wrong!");

        let guess: i32 = match guess.trim().parse::<i32>() { // try to parse the guess to an int
            Err(_) => { // on a failed parse
                println!("must be number!");
                continue;
            }
            Ok(num) => num // if valid just return the number to the guess object
        };

        match guess.cmp(&number) { // this is essentially a switch statement as there's no return value
            Ordering::Less => println!("you're guessing too low"), // match the return of .cmp() to a "type"(?)
            Ordering::Greater => println!("you're guessing too high"),
            Ordering::Equal => {
                println!("You win after {} attempts! the number was {}", guesses, number);
                break; // breaks the loop
            }
        }
        guesses += 1;
        println!()
    };
}
