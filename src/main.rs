use std::io;
//If a type you want to use isn’t in the prelude, 
//you have to bring that type into scope explicitly with a use statement. 
//Using the std::io library provides you with a number of useful features, 
//including the ability to accept user input.


fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new(); // a mutable variable that is currently bound to a new, empty instance of a String. 

    io::stdin() //The stdin function returns an instance of std::io::Stdin, which is a type that represents a handle to the standard input for your terminal.
         .read_line(&mut guess) //read_line is to take whatever the user types into 
         //standard input and append that into a string. also returns a "Result" value.
          //&mut is a mutable reference
         .expect("Failed to read line");

        // could write "io::stdin().read_line(&mut guess).expect("Failed to read line");


    println!("You guessed: {guess}");
}

// Code that gets a guess from the user and prints it!