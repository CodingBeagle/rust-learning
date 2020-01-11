// The Rust Standard library provides in input/output library
// Types not in the "prelude" will have to be explicitly brought into scope with
// "use" statements.
use std::io;
use std::cmp::Ordering;
use rand::Rng;

// "fn" syntax declares a new function.
fn main() {
    // Generate random secret number
    // thread_rng returns a random number generator that is current to the current thread
    // of execution and seeded by the operating system.
    // The gen_range method is defined by the Rng trait from use rand::Rng.
    // It takes two arguments, the inclusive lower bound and exclusive upper bound.
    // Thus 1 to 101 gives us a random number between 1 and 100.
    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("Welcome to Guess the Number!");

    loop {
        // The "let" syntax defines a new variable.
        // The "mut" modifier makes the variable mutable (they are immutable by default)
        // String::new() is a function that returns a new instance of a String.
        // String is a string type in the Rust standard library, which is a growable UTF-8 encoded
        // Bit of text.
        // The "::" syntax indicates that "new()" is an "associated function" of the String type.
        // An associated function is a function implemented on a type instead of a specific
        // Instance on that type. This is called a "static method" in other languages.
        let mut guess = String::new();

        println!("Please input your guess: ");

        // stdin() is a function that returns an instance of std::io::Stdin, which is a type that
        // Represents a handle to the standard input for your terminal.
        io::stdin()
            // read_line takes input from the standard input and places it into the string you provide
            // as a parameter. "&" indicates that the argument is a reference, meaning the parameter's
            // memory can be accessed in the function. References are, like variables, immutable by default,
            // Therefore, we have to write "&mut" to get a mutable reference of our string "guess".
            .read_line(&mut guess)
            // read_line returns a value of type io::Result.
            // Result is an enumeration. Result have the variants "Ok" or "Err"
            // "Err" means the operation failed.
            // An instance of "Result" as an "expect" method on it. If the value of Result is "Err",
            // calling expect causes the program to crash and display the message you passed as
            // argument.
            .expect("Failed to read line.");

        // Strings can be formatted in Rust with the "{}". This is called a placeholder.
        println!("You guessed: {}", guess);

        // Redefine "guess" as an integer so we can compare it to the secret number
        // Rust allows us to "Shadow" the previous value of a variable with a new one.
        // This lets you re-use the variable name!
        // The "parse" method parses a string into some kind of number. The method can parse a
        //a variety of number types, nad therefore we need to tell Rust the exact type by specifying
        // the type of the variable as "u32".
        // Parse returns a Result enum as parsing can fail.
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // The "cmp" method compares two values and can be called on anything
        // that can be compared. It takes a reference to whatever you want to compare with.
        // It then returns a variant of the "Ordering" enum.
        // The "match" expression is used to decide what to execute based on the result of the
        // Ordering enum.
        // A match expression is made up of "arms". An arm consists of a pattern and the code
        // that should be executed if the value given to the beginning of the match expression fits
        // that arm's pattern.
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break; // Break game loop
            }
        }
    }
}
