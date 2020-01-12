fn main() {
    string_allocation();
    string_ownership();
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