// this is an alias to use the io module
use std::io;

fn main() {
    println!("Guess the number!");
    println!("Please input a guess:");

    // definition of the guess variable as mutable
    let mut guess = String::new();
    // passing a mutable reference to guess, we need to pass a reference
    // because guess is not a primitive
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}
