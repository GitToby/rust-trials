use std::io;
use crate::games::rps::models::Move;
use crate::games::rps::models::Move::{Paper, Rock, Scissors};
use crate::games::rps::models::GameResult::{*};
use rand::Rng;

mod models;

pub fn play() {
    let mut random_generator = rand::thread_rng();
    let mut wins = 0;
    let mut draws = 0;
    let mut losses = 0;
    loop {
        println!("Rock paper scissors, GO!");
        println!("1) {:?}", Rock);
        println!("2) {:?}", Paper);
        println!("3) {:?}", Scissors);
        println!();

        let mut input: String = String::from("");

        let player_move: Move = match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let value = input.trim().parse::<i32>().unwrap_or( 0);
                match Move::from_int(value) {
                    Ok(result) => result,
                    Err(e) => {
                        println!("{}", e);
                        continue;
                    }
                }
            }
            Err(e) => {
                println!("{:?}", e);
                continue;
            }
        };


        let opponent = Move::from_int(random_generator.gen_range(1, 4)).unwrap();

        print!("You picked {:?}, up against {:?} ", player_move, opponent);

        match player_move.beats(&opponent) {
            Win => {
                println!("You Win!");
                wins += 1;
            }
            Lose => {
                println!("You Lose :(");
                losses += 1;
            }
            Draw => {
                println!("Draw");
                draws += 1;
            }
        };

        while !["y", "n"].contains(&input.trim()) {
            println!("again? (y/n)");
            input = "".to_string();
            io::stdin().read_line(&mut input).expect("failed :(");
        }

        if input.trim() == "n" {
            let total = wins + losses + draws;
            println!("Wins: {} of {} ({}%)", wins, total, 100 * wins / total);
            println!("Loses: {} of {} ({}%)", losses, total, 100 * losses / total);
            println!("Draws: {} of {} ({}%)", draws, total, 100 * draws / total);
            break;
        }
    }
}