use std::io;

fn main() {
    println!("Rock, paper, scissors, Type a move or type 'quit' to exit the game.");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    println!("You typed: {input}");
}
