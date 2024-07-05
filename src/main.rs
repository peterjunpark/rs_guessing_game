use std::io;

fn main() {
    println!("Guess the fucking number!");

    println!("Input your number now.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed {guess}");
}
