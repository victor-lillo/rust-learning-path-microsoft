fn main() {
    // Declare a constant. Must be typed.
    const NUMBER: u8 = 7;

    // Declare a variable
    let a_number;

    // Declare a second variable and bind the value
    let a_word = "Ten";

    // Bind a value to the first variable
    a_number = 10;

    println!("The constant number is {NUMBER}.");
    println!("The number is {}.", a_number);
    println!("The word is {}.", a_word);

    // ! Inmutable vs mutable
    // The `mut` keyword lets the variable be changed
    let mut a_number = 10;
    println!("The number is {}.", a_number);

    // Change the value of an immutable variable
    a_number = 15;
    println!("Now the number is {}.", a_number);

    // Declare first variable binding with name "shadow_num"
    let shadow_num = 5;

    // ! Variable shadowing
    // Declare second variable binding, shadows existing variable "shadow_num"
    let shadow_num = shadow_num + 5;

    // Declare third variable binding, shadows second binding of variable "shadow_num"
    let shadow_num = shadow_num * 2;

    println!("The number is {}.", shadow_num);
}
