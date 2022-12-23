fn main() {
    println!("Hello, world!");
}

// The question mark symbol (?) after that statement is used to propagate errors without writing too much boilerplate code. It's syntax sugar for early returning an error if that error matches with the return type of the function it's in. So these snippets are equivalent:
fn function_1() -> Result(Success, Failure) {
    match operation_that_might_fail() {
        Ok(success) => success,
        Err(failure) => return Err(failure),
    }
}

fn function_2() -> Result(Success, Failure) {
    operation_that_might_fail()?
}
