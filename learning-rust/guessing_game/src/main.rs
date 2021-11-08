// bring the io (input/output) library into scope
use std::io;
// Rng trait defines methods that random number generators implement
use rand::Rng;
// 
use std::cmp::Ordering;

fn main() {
    //  a macro that prints a string to the screen
    println!("Guess the number!");

    // the particular random number generator that we’re going to use
    let secret_number = rand::thread_rng().gen_range(1..101);

    println!("The secret number is: {}", secret_number);

    // the loop keyword creates an infinite loop
    loop {

      println!("Please input your guess.");

      // let statement, which is used to create a variable
      // mut before the variable name to make a variable mutable
      // String::new, a function that returns a new instance of a String
      let mut guess = String::new();

      // call the stdin function from the io module
      io::stdin()
          // calls the read_line method on the standard input handle
          // & indicates that this argument is a reference, references are immutable by default
          .read_line(&mut guess)
          // it also returns a value—in this case, an io::Result
          // An instance of io::Result has an expect method that you can call
          .expect("Failed to read line");

      // We create a variable named guess
      // Rust allows us to shadow the previous value of guess with a new one
      // The trim method on a String instance will eliminate any whitespace at the beginning and end
      // The parse method on strings parses a string into some kind of number
      // Switching from an expect call to a match expression is one way of moving from crashing on an error to handling the error.
      let guess: u32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => {
          println!("Invalid value");
          continue;
        }
      };

      //  The set of curly brackets, {}, is a placeholder
      println!("You guessed: {}", guess);

      match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        // Adding the break line after You win! makes the program exit the loop when the user guesses the secret number correctly
        Ordering::Equal => {
          println!("You win!");
          break;
        }
      }
  }
}