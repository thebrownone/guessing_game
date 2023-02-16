use rand::Rng; 
//Rng is a trait that defines methods that random number generators implement,
use std::cmp::Ordering; 
//Ordering is an enum that can be either Less, Greater, or Equal.
use std::io;
//If a type you want to use isn’t in the prelude, 
//you have to bring that type into scope explicitly with a use statement. 
//Using the std::io library provides you with a number of useful features, 
//including the ability to accept user input.

fn main() {
    println!("Guess the number!");

    let secret_number= rand::thread_rng().gen_range(1..= 100); 
    //thread_rng is a function that returns the particular random number generator that we’re going to use: one that is local to the current thread of execution and seeded by the operating system.
    //gen_range is a method on the Rng trait that takes two numbers as arguments and generates a random number between them.

    loop {

        println!("Please input your guess.");
    
        let mut guess = String::new(); 
        // a mutable variable that is currently bound to a new, empty instance of a String. 
    
        io::stdin() //The stdin function returns an instance of std::io::Stdin, which is a type that represents a handle to the standard input for your terminal.
             .read_line(&mut guess) //read_line is to take whatever the user types into 
             //standard input and append that into a string. also returns a "Result" value.
              //&mut is a mutable reference
             .expect("Failed to read line");
    
            // could write "io::stdin().read_line(&mut guess).expect("Failed to read line");
    
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        //trim() removes whitespace
        //parse() converts string to number
    
        println!("You guessed: {guess}");
    
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
