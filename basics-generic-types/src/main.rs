#![allow(dead_code, unused_variables)]
struct Point<T> {
    x: T,
    y: T,
}
struct PointMultiple<T, U> {
    x: T,
    y: U,
}

fn main() {
    let boolean = Point { x: true, y: false };
    let integer = Point { x: 1, y: 9 };
    let float = Point { x: 1.7, y: 4.3 };
    let string_slice = Point {
        x: "high",
        y: "low",
    };

    let integer_and_boolean = PointMultiple { x: 5, y: false };
    let float_and_string = PointMultiple { x: 1.0, y: "hey" };
    let integer_and_float = PointMultiple { x: 5, y: 4.0 };
    let both_integer = PointMultiple { x: 10, y: 30 };
    let both_boolean = PointMultiple { x: true, y: true };
}
