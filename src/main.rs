use std::io;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

enum Outcome {
    Win,
    Lose,
    Draw,
}

fn main() {
    let mut wins = 0;
    let mut losses = 0;
    let mut draws = 0;

    loop {
        println!("Your move? (rock, paper, scissors, or quit?)");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        if input.trim().eq_ignore_ascii_case("quit") {
            println!("Thanks for playing!");
            break;
        }

        let player = match parse_move(&input) {
            Some(mv) => mv,
            None => {
                println!("Invalid move. Please try again.");
                continue;
            }
        };


        let computer = random_move();
        println!("Computer chose: {:?}", computer);

        match judge(player, computer) {
            Outcome::Win => {
                println!("You win!");
                wins += 1;
            }
            Outcome::Lose => {
                println!("You lose!");
                losses += 1;
            }
            Outcome::Draw => {
                println!("It's a draw!");
                draws += 1;
            }   
        }



    }
    println!("Final score: {} wins, {} losses, {} draws", wins, losses, draws);
}

fn random_move() -> Move {
    match rand::random_range(0..3) {
        0 => Move::Rock,
        1 => Move::Paper,
        _ => Move::Scissors,
    }
}

fn parse_move(text: &str) -> Option<Move> {
    match text.trim().to_lowercase().as_str() {
        "rock" | "r" => Some(Move::Rock),
        "paper" | "p" => Some(Move::Paper),
        "scissors" | "s" => Some(Move::Scissors),
        _ => None,
    }
}

fn judge(player: Move, computer: Move) -> Outcome {
    use Move::{Paper, Rock, Scissors};
    match (player, computer) {
        (Rock, Scissors) | (Paper, Rock) | (Scissors, Paper) => Outcome::Win,
        (a, b) if a == b => Outcome::Draw,
        _ => Outcome::Lose,
    }
}
