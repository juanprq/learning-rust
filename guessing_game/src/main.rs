// this is an alias to use the io module
use std::io;

fn main() {
    println!("Guess the number!");
    println!("Please input a guess:");

    // definition of the guess variable as mutable
    // the :: syntax means an asociated function to a type
    let mut guess = String::new();
    // passing a mutable reference to guess, we need to pass a reference
    // because guess is not a primitive
    io::stdin()
        .read_line(&mut guess)
        // the way that expect works, is that read_line returns a Result type
        // in case the Result is an error, the expect method will crash the progam
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}
