// this is an alias to use the io module
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("The secret number is: {}", secret_number);

    // definition of the guess variable as mutable
    // the :: syntax means an asociated function to a type
    loop {
        let mut guess = String::new();
        // passing a mutable reference to guess, we need to pass a reference
        // because guess is not a primitive
        println!("Please input a guess:");
        io::stdin()
            .read_line(&mut guess)
            // the way that expect works, is that read_line returns a Result type
            // in case the Result is an error, the expect method will crash the progam
            .expect("Failed to read line");

        // we parse the guess to u32
        // we use shadowing to redefine the guess variable
        // also we need to specify the u32 type annotation in order to indicate to
        // the compiler, that we are parsing to u32
        let guess: u32 = guess.trim().parse().expect("Please input your guess:");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => println!("You win!"),
        }
    }
}
