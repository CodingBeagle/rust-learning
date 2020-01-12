// consts are different from immutable variables
// They can be declared in the constant scope
// And you are REQUIRED to specify a type.
// Also, a value must not be a result of any evaluated expression.
const MAX_POINTS: u32 = 100;

fn main() {
    println!("The value MAX_POINTS is: {}", MAX_POINTS);

    // Variables are, by default, immutable.
    let x = 5;
    println!("The value of x is: {}", x);

    // Meaning this isn't allowed. It cannot be re-assigned to another value.
    // x = 6;

    // If you want mutable variables, use the "mut" keyword.
    // "mut" indicates intent to readers that this variable can and probably will change
    // at some point in the program's lifetime.
    let mut y = 10;
    println!("The value of y is: {}", y);

    // Now we can change it after the fact!
    y = 20;
    println!("The value of y is now: {}", y);

    shadowing();
}

fn shadowing() {
    // Rust has the concept of "shadowing".
    // This means that a variable can be declared using the name of an already existing
    // variable name in the same scope.
    // Even immutable variables, as you're not re-assigning the value of the variable,
    // You're essentially creating a new variable
    let local_variable = 10;
    println!("The value of localVariable is: {}", local_variable);

    // Not only can you re-assign the value, you can change the type
    let local_variable = "Now I'm a string! :o";
    println!("The value of localVariable is: {}", local_variable);
}
