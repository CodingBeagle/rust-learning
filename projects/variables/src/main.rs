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
    integers();
    floating_points();
    character_type();
    tuple_type();
    array_type();
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

fn integers() {
    // Rust's integers comes in signed or unsigned versions
    // You have 8, 16, 32, 64, and 128-bit integers
    // Rust defaults to u32.
    let unsigned_integer: u64 = 10;
    let signed_integer: i64 = -50;

    println!("The value of unsignedInteger is {}", unsigned_integer);
    println!("The value of signedInteger is {}", signed_integer);
}

fn floating_points() {
    // Rust has the floating-point types f32 and f64.
    // The default type is f64.
    let x = 2.0; // f64
    let y: f32 = 3.0; // f32

    println!("Value of float x is {}", x);
    println!("Value of float y is {}", y);
}

fn character_type() {
    // The Char type is the language's most primitive alphabetic type.
    // Char literals are specified with single quotes ''
    // Rust's char type is 4 bytes in size
    // They represent Unicode Scalar Values.
    let c = 'z';
    let z = 'Z';
    let heart_eyed_cat = '😻';

    println!("Value of character c is {}", c);
    println!("Value of character z is {}", z);
    println!("Value of character heart_eyed_cat is {}", heart_eyed_cat);
}

fn tuple_type() {
    // The Tuple type is one of Rust's two compound types
    // It's good for grouping together a number of values with a variety of types into one compound type.
    let my_tuple: (i32, f64, u8) = (500, 6.4, 1);

    // To get individual values, we use pattern matching to destructure a tuple value
    let (x, y, z) = my_tuple;

    println!("Value of x is {}", x);
    println!("Value of y is {}", y);
    println!("Value of z is {}", z);

    // But values can also be accessed using dot notation
    println!("Value of x through dot notation is {}", my_tuple.0);
}

fn array_type() {
    // Unlike a tuple, all values of an array must have the same type.
    // Arrays in Rust have a fixed length!
    // Arrays are useful when you want data allocated on the stack rather than the heap.
    let my_simple_array = [1, 2, 3, 4, 5];

    // Accessing a value in an array can be done through indexing
    println!("Value of second element is: {}", my_simple_array[1]);
}