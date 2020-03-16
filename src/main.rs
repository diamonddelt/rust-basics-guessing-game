extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    // println! is a "macro"
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 201);

    // debug - uncomment if you want ezmode
    // println!("The secret number is: {}", secret_number);

    // create an infinite loop - nice
    loop {
        println!("Please input your guess:"); 
    
        // String is a type and :: acts on the type directly
        let mut guess = String::new();
    
        // Java equivalent: Random.rand()
        // use an associated function of the type
        // .expect() explicitly handles the ::Result emitted
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");
    
        // use Rust's version of type casting and handle a potential error
        // annotate the guess variable as a u32 type (shadowing)
        // uses match ::Result to determine if the cast succeeded
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num, // if match succeeds, set unwrapped Ok value to num
            Err(_) => {
                // if match fails (with Err); next iter of loop
                println!("Non-numeric entry detected.");
                continue;
            },
        };
    
        println!("You guessed: {}", guess);
    
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("LOW"),
            Ordering::Greater => println!("HIGH"),
            Ordering::Equal => {
                // exit
                println!("MATCH!");
                break;
            }
        }
    }
}
