fn main() {
    string_allocation();
    string_ownership();
    stack_ownership();

    // Ownership functions
    let hello_world = String::from("Hello, World!");
    println!("Now I'm printing my string: {}", hello_world);

    takes_ownership(hello_world);

    // This line would not be valid! hello_world string was moved in function call!
    // println!("{}", hello_world);

    let my_number = 10;
    makes_copy(my_number);

    println!("My Number is: {}", my_number);

    // Return value ownership
    {
        let my_returned_string = gives_ownership();
        println!("It's my string! It's miine! {}", my_returned_string);

        let an_awesome_string = String::from("Awesome string!");

        let new_awesome_string = gives_ownership_and_takes_back(an_awesome_string);

        println!("Ownership is now new_awesome_string {}", new_awesome_string);


    }
}

fn string_allocation() {
    {
        // String::from actually allocates a mutable string, which means its stored on the heap!
        let my_string = String::from("I'm a heap string!");
        println!("The value of my_string is: {}", my_string);
    }
    // Rule of thumb with memory management is that you must pair one "new" with one "delete"!
    // But here we don't do anything to clean up the string.
    // Rust does this for us by calling a special function called "Drop" at the end of it's scope.
    // Drop is like a destructor in C++, following the RAII pattern.
}

fn string_ownership() {
    // In Rust, a String on the heap is made up of a pointer to the stack, as well as its
    // Length and Capacity.
    let s1 = String::from("hello");

    // When we assign s1 to s2, the STACK data is copied, meaning a pointer to the string on the
    // heap. This means s1 and s2 are actually pointers pointing to the same heap memory!
    let s2 = s1;

    // You can't actually do this now! s1 has been deemed invalid by the Rust borrow checker.
    // Since you assigned s1 to s2, meaning shared data, Rust would have tried to free both
    // s1 and s2 at the end of the function scope. This would lead to a classic "double free error".
    // Trying to remove memory already freed.
    // This is known as a "move". s1 was MOVED into s2.
    // println!("Value of s1 is: {}", s1);
}

fn string_ownership_clone() {
    let s1 = String::from("hello!");
    let s2 = s1.clone();
}

fn stack_ownership() {
    // This is allowed
    let x = 5;
    let y = x;

    // This is because types that are stored on the stack (such as integers),
    // can have actual copies of their data taken, meaning you get two separate values.
    // Ruast has a special annotation called the "Copy" trait. It is used on types like
    // Integers, or custom types that will be stored on the stack.
    // You cannot annotate a type with the Copy trait if it also has the Drop trait.
    // These in-built types are already "Copy" trait:
    // - All integer types
    // - The boolean type
    // - All floating point types
    // - The character type
    // - Tuples. But ONLY if they contain types that are also "Copy".
    println!("I can use both variables! {}, {}", x, y);
}

// For functions, passing a variable will either move or copy just like assignment.
fn takes_ownership(some_string: String) {
    println!("{}", some_string);
} // some_string goes out of scope and "drop" is called. The underlying memory is freed.

fn makes_copy(some_integer: u32) {
    println!("{}", some_integer);
} // some_integer goes out of scope, but nothing special happens.

// For return values of functions, ownership can also be transferred.
fn gives_ownership() -> String {
    let hello_world = String::from("Hello, World! :D");
    hello_world
}

fn gives_ownership_and_takes_back(some_string: String) -> String {
    some_string
}