// use is similar to import in other languages
use std::io;
pub fn run() {
    // mut means mutable, meaning we can change the value later
    let mut guess = String::new();

    println!("Welcome to ~Guess the number!~");
    println!("Please input your guess.");

    io::stdin()
        // & means reference, meaning we're using the same value
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}
