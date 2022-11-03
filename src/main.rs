// use is similar to import in other languages
use std::io;
// mod to import modules from other files
mod guessing_game;

// clear screen function
fn clear_screen() {
    print!("{}[2J", 27 as char);
}

fn main() {
    clear_screen();

    // list down all the modules and let the user to select which one to run
    println!("Which module do you want to run?");
    println!("1. Guessing Game");
    println!("2. Exit");

    let mut input: String = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let input: u32 = input.trim().parse().expect("Please type a number!");

    match input {
        1 => {
            clear_screen();
            guessing_game::run();
        }
        2 => println!("Exiting..."),
        _ => println!("Invalid input!"),
    }
}
