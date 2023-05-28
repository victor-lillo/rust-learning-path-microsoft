fn main() {
    // &str (String Slice): inmutable
    // https://doc.rust-lang.org/book/ch04-03-slices.html
    let hello: &str = "Hola";
    let world: &str = "Mundo";
    let hello_world = hello.to_owned() + " " + world;
    println!("{}", hello_world);

    // String: mutable
    let str: &str = "Hola";
    let mut hello = String::from(str);
    hello.push_str(", Mundo!");
    println!("{hello}");
}
