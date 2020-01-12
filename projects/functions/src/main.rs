fn main() {
    a_function(555);
    multiplication(5, 5);

    let function_result = multiplication2(10, 10);
    println!("Result of multiplication is: {}", function_result);

    println!("When not multiplying: {}", multiplication3(5, 5, false));
}

// Functions in Rust can have parameters...
// IN Rust, you are required to define parameter types explicitly.
fn a_function(x: i32) {
    println!("Hello! The number you passed has a value of: {}", x);
}

fn multiplication(x: i32, y: i32) {
    let result = x * y;
    println!("The multiplication result is: {}", result);
}

// Functions in Rust can return values, which can be done in multiple ways
fn multiplication2(x: i32, y: i32) -> i32 {
    // Functions in Rust can have values returned implicitly through the last expression of
    // The function body! Notice that there's no semicolon here. This makes it an expression
    // The value of which is returned from the function to the caller.
    x * y
}

fn multiplication3(x: i32, y: i32, should_multiply: bool) -> i32 {
     if should_multiply == false {
         // You can also return early from functions using the "return" keyword
         return 0
     }

    x * y
}