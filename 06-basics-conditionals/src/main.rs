fn check_is_odd_or_even(number: i32) {
    // 'if' is an expression, it returns data
    let result = if number % 2 == 0 { "par" } else { "impar" };
    println!("El número es {result}");
}

fn return_is_odd_or_even(number: i32) -> String {
    match number % 2 {
        0 => "par".to_string(),
        1 => "impar".to_string(),
        _ => "no es ni par ni impar".to_string(), // Placeholder
    }
}

fn main() {
    // ! If
    let number = 5;
    if number > 5 {
        println!("El número es mayor que 5");
    } else if number < 5 {
        println!("El número es menor que 5");
    } else {
        println!("El número es 5");
    }

    let number_2 = 5;
    check_is_odd_or_even(number_2);

    // ! Switch
    let switch_result = return_is_odd_or_even(number_2);
    println!("El número {number_2} es {switch_result}")
}
