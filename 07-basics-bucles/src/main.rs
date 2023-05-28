fn loop_function() {
    let mut number = 1;

    let string_result = loop {
        number += 1;

        if number == 5 {
            // Can return a value
            break "Loop: es 5";
        }
    };

    println!("{string_result}");
}

fn while_function() {
    let mut number = 0;

    while number < 5 {
        println!("While: es {number}");
        number += 1;
    }
}

fn for_in_function() {
    let numbers = [1, 2, 3, 4, 5];

    // for number in 1..6 {
    // for number in 1..=5 {
    for number in numbers {
        println!("For in: es {number}");
    }
}

fn main() {
    loop_function();
    while_function();
    for_in_function();
}
