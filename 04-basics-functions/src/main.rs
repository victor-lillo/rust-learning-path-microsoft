// snake_case recommended
fn multiply_by2(number: i32) -> i32 {
    return number * 2;
}

// Implicit return, no ; at the end
fn multiply_by3(number: i32) -> i32 {
    number * 3
}

// No return, let Rust infer or '-> ()'
fn no_return(number: i32) -> () {
    println!("Esta función imprime el número {number}");
}

fn main() {
    const NUMBER: i32 = 5;

    let resultx2 = multiply_by2(NUMBER);
    println!("El doble de {NUMBER} es {resultx2}");

    let resultx3 = multiply_by3(NUMBER);
    println!("El triple de {NUMBER} es {resultx3}");

    no_return(NUMBER)
}
