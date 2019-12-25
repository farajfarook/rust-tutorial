use std::io;

fn main() {
    println!("Guess a word");
    let mut word = String::new();
    io::stdin().read_line(&mut word).expect("failed to read");

    println!("You guessed {}", word)
}
