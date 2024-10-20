use std::io;                // Lesson 2
use rand::Rng;              // Lesson 2
use std::cmp::Ordering;     // Lesson 2

enum Context {
    None,
    Side
}

fn main() {
    println!("Lesson 1: Complete");
    lesson2();
}

fn set_context(context: Context) {
    match context {
        Context::None => print!("\x1b[0m"),
        Context::Side => print!("\x1b[38;5;240m")
    }
}

// Lesson 1: Getting Started
// The Cargo Project was completed as you're looking at it.

// Lesson 2: Program a Guessing Game
fn lesson2() {
    set_context(Context::Side);

    // Generate a Secret Number between 1-9
    println!("Guess a number... (1 - 9)");
    let secret_number = rand::thread_rng().gen_range(1..=9);

    // Game Loop of Guess Number
    loop {
        // Make a blank mutable string
        let mut guess: String = String::new();
        // Grab the input
        io::stdin() 
            .read_line(&mut guess)
            .expect("Failed to read line");
        // Make sure input is a number, otherwise re-loop
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        // Feeback of High/Low Break on success
        match guess.cmp(&secret_number) {
            Ordering::Greater => println!("Too High"),
            Ordering::Less => println!("Too Low"),
            Ordering::Equal => {
                println!("You've Won!");break;
            }
        }
    }
    set_context(Context::None);
    println!("Lesson 2: Complete");
}