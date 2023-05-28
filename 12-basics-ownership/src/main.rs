/*
Ownership rules
- Each value in Rust has an owner.
- There can only be one owner at a time.
- When the owner goes out of scope, the value will be dropped.
 */

fn main() {
    // Scalar types can Copy: integers, bool, floating, chars.
    // Also tuples with only scalar. e.g.: (i32, i32)
    let num = 5.7;
    let num2 = num; // Copied
    println!("{num} {num2}"); // -> 5 5

    copy_variable()
}

fn to_lowercase(name: String) -> String {
    name.to_lowercase()
}

fn copy_variable() {
    let s1 = String::from("Hello");
    // let s2 = s1; -> Not cloned, just change ownership (move)
    let s2 = s1.clone();
    println!("{} {}", s1, s2);

    // The same happens when we pass the variable to a method
    let lower_case_name = to_lowercase(s1.clone());

    println!("{lower_case_name}");
}
