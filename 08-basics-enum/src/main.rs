#![allow(dead_code)]
#[derive(Debug)]
enum Color {
    Red, // Static variant
    Green,
    Blue,
    Rgb(u8, u8, u8), // Dynamic variant
}

fn main() {
    let red = Color::Red;
    let dark_blue = Color::Rgb(0, 0, 100);

    println!("{:#?}{:#?}", red, dark_blue);
}
