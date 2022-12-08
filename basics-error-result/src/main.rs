#[derive(Debug)]
struct CustomDivisionByZeroError;

// Result is defined as:
// enum Result<T, E> {
//     Ok(T):  // A value T was obtained.
//     Err(E): // An error of type E was encountered instead.
// }

fn safe_division(dividend: f64, divisor: f64) -> Result<f64, CustomDivisionByZeroError> {
    if divisor == 0.0 {
        Err(CustomDivisionByZeroError)
    } else {
        Ok(dividend / divisor)
    }
}

fn main() {
    println!("{:?}", safe_division(9.0, 3.0));
    println!("{:?}", safe_division(4.0, 0.0));
    println!("{:?}", safe_division(0.0, 2.0));
}
