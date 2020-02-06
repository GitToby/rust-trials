use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guessing Game!");
    println!("--------------");

    let number: i32 = rand::thread_rng().gen_range(1, 1001);
    let mut guesses: i32 = 0;

    loop {
        println!("Guess the number:");
        let mut guess: String = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Something went wrong!");

        let guess: i32 = guess.trim().parse::<i32>()
            .expect("expected a number");

        match guess.cmp(&number) {
            Ordering::Less => println!("you're guessing too low"),
            Ordering::Greater => println!("you're guessing too high"),
            Ordering::Equal => {
                println!("You win after {} attempts! the number was {}", guesses, number);
                break;
            }
        }
        guesses += 1;
        println!()
    };
}
