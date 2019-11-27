use rand::Rng;
use std::io;
use std::io::Write;

const ROCK: &str = "rock";
const PAPER: &str = "paper";
const SCISSOR: &str = "scissor";

const WIN: i32 = 1;
const DRAW: i32 = 0;
const LOSE: i32 = -1;

const BEST_OF: i32 = 5;

fn generate_computer_move() -> &'static str {
    let computer_move = rand::thread_rng().gen_range(1, 3);

    match computer_move {
        1 => return ROCK,
        2 => return PAPER,
        3 => return SCISSOR,
        _ => panic!("Error generating computer move!"), // impossible
    }
}

fn validate_move(user_move: &str) -> bool {
    if user_move == ROCK || user_move == PAPER || user_move == SCISSOR {
        return true;
    } else {
        return false;
    }
}

fn evaluate_move(user_move: &str, computer_move: &str) -> i32 {
    if user_move == ROCK {
        if computer_move == ROCK {
            return DRAW;
        } else if computer_move == PAPER {
            return LOSE;
        } else {
            return WIN;
        }
    } else if user_move == PAPER {
        if computer_move == ROCK {
            return WIN;
        } else if computer_move == PAPER {
            return DRAW;
        } else {
            return LOSE;
        }
    } else {
        if computer_move == ROCK {
            return LOSE;
        } else if computer_move == PAPER {
            return WIN;
        } else {
            return DRAW;
        }
    }
}

fn rps_round() -> i32 {
    let computer_move = generate_computer_move();
    // println!("Computer move: {}", computer_move);

    loop {
        print!("Your move: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line.");

        let user_move = input.trim();

        if validate_move(user_move) {
            let result = evaluate_move(user_move, computer_move);
            return result;
        }
    }
}

fn main() {
    let mut user_score = 0;
    let mut computer_score = 0;

    loop {
        let result = rps_round();

        match result {
            1 => {
                user_score += 1;
                println!("You score: {}-{}", user_score, computer_score)
            }
            0 => println!("It's a draw: {}-{}", user_score, computer_score),
            -1 => {
                computer_score += 1;
                println!("Computer score: {}-{}", user_score, computer_score);
            }
            _ => panic!("Error announcing result!"), // impossible
        }

        if user_score == BEST_OF {
            println!("You Win");
            break;
        }

        if computer_score == BEST_OF {
            println!("You Lose");
            break;
        }
    }
}
