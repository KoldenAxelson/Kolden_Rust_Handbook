use std::io;                // Lesson 2
use rand::Rng;              // Lesson 2
use std::cmp::Ordering;     // Lesson 2

enum Context {
    None,
    Side
}

const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3; // Lesson 3

fn main() {
    println!("Lesson 1: Complete");
    lesson2();
    lesson3();
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

// Lesson 3: Common Programming Concepts
fn lesson3() {
    set_context(Context::Side);

    // Mutating Variables must be declared with mut
    let mut x = 5;
    print!("X={x} => ");
    x = 6;
    println!("X={x}");

    // Constants can be declared in global scope
    println!("{}", THREE_HOURS_IN_SECONDS);

    // Shadow in sub-scoping
    {
        let x = 7;
        print!("X={x} => ");
    }
    println!("X={x}");

    // Rewriting Varibales
    let x = "12345";
    let x = x.len();
    println!("String '12345' => {x}");

    // Strict Typing
    // Sometimes the ": u32" is needed
    // ITC: What value to be parsing out
    let x: u32 = "42".parse().expect("Not a number!");
    println!("String '42' => {x}");

    // Number Sizes
    // Signed(i)/Unsigned(u) 8,16,32,64,128,size(32x/64x)
    let x: f32 = 20.0;
    println!("Floating Point: {x}");

    // Operations
    let mut x = 100 + 50;
    x = x - 50;
    x = x * 2;
    x = x / 2;
    x = x % 3;
    println!("100+50-50*2/2%3={x}");

    // Primitives
    let x: bool = true; //or false
    let c: char = '🥳';
    if x {print!("{c}");}

    // Tuple (Sets of different types)
    let x: (bool, char) = (x,c);
    if x.0 {println!("{}",x.1);}

    // Array (Sets of same types)
    let x: [&str; 2] = ["Kolden", "Axelson"];
    println!("{} {}",x[0],x[1]);

    // Functions
    println!("{}", lesson3_helper(5));

    // Calculated Assignments & Implied Returns
    let x = {
        let y = 3;
        y + 1
    };
    println!("X={x}");

    // Loops
    let c = 'x';

    let mut x: String = String::new();
    loop {
        x = format!("{x}{c}");
        if x.len() >= 5 {break;}
    }
    println!("Loop: {x}");

    let mut x: String = String::new();
    while x.len() < 5 {
        x = format!("{x}{c}");
    }
    println!("While: {x}");

    let mut x: String = String::new();
    for _ in 0..5 {
        x = format!("{x}{c}");
    }
    println!("For: {x}");

    // Finish Lesson
    set_context(Context::None);
    println!("Lesson 3: Complete");
}

fn lesson3_helper(x: usize) -> usize {
    print!("{x} => ");
    return x * 2;
}