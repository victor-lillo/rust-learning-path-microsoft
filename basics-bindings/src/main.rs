fn process_move(input: String) {
    println!("Move: {}", input)
}

// In this case, 's' variable is moved, cause copying it would be expensive
fn caller_moved() {
    let s = String::from("Hello, world!");
    process_move(s); // Ownership of the string in `s` moved into `process`
                     // process_move(s); // Error! ownership already moved.
}

fn process_copy(input: u32) {
    println!("Copy: {}", input)
}

// In this case, 'n' variable is copied, cause is inexpensive
fn caller_copied() {
    let n = 1u32;
    process_copy(n); // Ownership of the number in `n` copied into `process_c`
    process_copy(n); // `n` can be used again because it wasn't moved, it was copied.
}

fn process_clone(input: String) {
    println!("Clone: {}", input)
}

fn caller_clone() {
    let str = String::from("Hello, world!");
    process_clone(str.clone()); // Passing another value, cloned from `s`.
    process_clone(str); // s was never moved and so it can still be used.
}

fn main() {
    caller_copied();
    caller_moved();
    caller_clone();
}
