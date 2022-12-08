fn main() {
    let fruits = vec!["banana", "apple", "coconut", "orange", "strawberry"];

    // pick the first item:
    let first = fruits.get(0);
    println!("{:?}", first);

    // pick the third item:
    let third = fruits.get(2);
    println!("{:?}", third); // Some("coconut")

    // pick the 99th item, which is non-existent:
    let non_existent = fruits.get(99);
    println!("{:?}\n", non_existent);

    //* Pattern matching
    for &index in [0, 1, 2, 99].iter() {
        match fruits.get(index) {
            Some(&"coconut") => println!("Coconuts are awesome!!!"),
            Some(fruit_name) => println!("It's a delicious {}!", fruit_name),
            None => println!("There is no fruit! :(\n"),
        }
    }

    //* If let expression
    let a_number: Option<u8> = Some(7);
    match a_number {
        Some(7) => println!("That's my lucky number!"),
        _ => {} // Wildcard. Satisfies compiler demands: with this, we ignore the None variant and all Some<u8> values don't match Some(7)
    }

    // If we need just one match, we can use if let
    if let Some(7) = a_number {
        println!("That's my lucky number!");
    }

    //* unwrap y expect -> not recommended, they can panick
    // Unwrap: access the value of an Option
    // Expect: the same, but the second argument is the custom panick message
    let gift = Some("candy");
    assert_eq!(gift.unwrap(), "candy"); // Assert that two expressions are equal

    // let empty_gift: Option<&str> = None;
    // assert_eq!(empty_gift.unwrap(), "candy"); // This will panic!

    //  With custom message
    let a = Some("value");
    assert_eq!(a.expect("fruits are healthy"), "value");

    // let b: Option<&str> = None;
    // b.expect("fruits are healthy"); // panics with `fruits are healthy`

    //* Recommended way:
    // Use pattern matching and handle the None case explicitly.
    // Call similar non-panicking methods, such as unwrap_or, which returns a default value if the variant is None or the inner value if the variant is Some(value).
    assert_eq!(Some("dog").unwrap_or("cat"), "dog");
    assert_eq!(None.unwrap_or("cat"), "cat");
}
